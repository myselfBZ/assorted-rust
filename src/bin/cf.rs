
enum Unit {
    Cel,
    Farn
}

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unit::Cel => write!(f, "C"),
            Unit::Farn => write!(f, "F")
        }
    }
}




fn main() {
    let mut user_input = String::new();
    // f for Farenheit C for Celcius
    let mut cel_or_far = String::new();
    println!("Press c to convert from Celcius and press f for vice versa");
    std::io::stdin().read_line(&mut cel_or_far).expect("failed to read");

    let cel_or_far = cel_or_far.trim();

    if cel_or_far != "c" && cel_or_far != "f" {
        println!("please pick between c or f, other charachters are not allowed");
        return
    }

    std::io::stdin().read_line(&mut user_input).expect("failed to read");

    let valid_number :f32 = user_input.trim().parse().expect("Only numbers are allowed");

    let unit_to =  if cel_or_far == "c" {
        Unit::Farn
    } else {
        Unit::Cel
    };


    let unit_from = if cel_or_far == "c" {
        Unit::Cel
    } else {
        Unit::Farn
    };

    let converted = convert(unit_from, valid_number);

    println!("Result: {converted}° {unit_to}")
}


fn convert(unit_from: Unit, val: f32) -> f32 {
    match unit_from {
        Unit::Cel => {
            val * (9.0 / 5.0) + 32.0
        },

        Unit::Farn => {
            (val - 32.0) * 5.0 / 9.0
        }
    }
}

