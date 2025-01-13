use layout::trait_alias;

trait_alias!(pub unsafe trait Concurrent: Send, Sync);
