enum Partner {
    Airline(String),
    Hotel(String),
}

struct Flight {
    airline: Partner::Airline,
    miles: u16,
    price: u16,
}

struct Stay {
    hotel: Partner::Hotel,
    price: u16,
}