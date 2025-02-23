use domain::ids::MachineId;

pub trait MachineIdFactory {
    fn getMachineId(&self) -> MachineId;
}
