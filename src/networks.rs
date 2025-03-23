

enum SignalRange {
    Good(usize),
    Average(usize),
    Bad(usize)
}

enum NetworkSecurity {
    Psk,
    Other(String)
}

pub struct Network {
    name: String,
    security: NetworkSecurity,
    Signal: SignalRange
}
