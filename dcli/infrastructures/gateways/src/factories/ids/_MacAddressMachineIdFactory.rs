use domain::ids::MachineId;
use sha2::Digest;
use sha2::Sha256;
use use_cases::gateways::factories::ids::MachineIdFactory;

pub struct MacAddressMachineIdFactory {
    precomputedMachineId: MachineId,
}

impl MachineIdFactory for MacAddressMachineIdFactory {
    fn getMachineId(&self) -> MachineId {
        return self.precomputedMachineId;
    }
}

impl MacAddressMachineIdFactory {
    pub fn new() -> Result<Self, MacAddressMachineIdFactoryError> {
        let machineId = Self::computeMachineId()?;
        return Ok(Self {
            precomputedMachineId: machineId,
        });
    }
    
    fn computeMachineId() -> Result<MachineId, MacAddressMachineIdFactoryError> {
        return Self::getMacAddress()
            .ok_or(MacAddressMachineIdFactoryError::MacAddressNotFound)
            .map(Self::hashMacAddress)
            .map(Self::extractTwoLeftmostBytesFromHashedMacAddress)
            .map(Self::newMachineIdFromTwoLeftmostBytes);
    }
    
    fn getMacAddress() -> Option<MacAddress> {
        return pnet_datalink::interfaces()
            .into_iter()
            .find(|networkInterface| networkInterface.mac
                .is_some())
            .and_then(|networkInterface| networkInterface.mac
                .map(|macAddress| macAddress.octets().to_vec()));
    }

    fn hashMacAddress(macAddress: MacAddress) -> HashedMacAddress {
        return Sha256::digest(&macAddress).into();
    }

    fn extractTwoLeftmostBytesFromHashedMacAddress(hashedMacAddress: HashedMacAddress) -> [u8; 2] {
        let [leftmostByte, secondLeftmostByte, ..] = hashedMacAddress;
        return [leftmostByte, secondLeftmostByte];
    }

    fn newMachineIdFromTwoLeftmostBytes(twoLeftmostBytes: [u8; 2]) -> MachineId {
        return MachineId::new(u16::from_ne_bytes(twoLeftmostBytes));
    }
}

pub enum MacAddressMachineIdFactoryError {
    MacAddressNotFound,
}

type MacAddress = Vec<u8>;
type HashedMacAddress = [u8; 32];
