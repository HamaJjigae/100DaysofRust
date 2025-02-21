use std::io;

#[derive(PartialEq)]
enum VehicleType {
    Car(i8),
    Motorcycle(bool),
}

#[derive(PartialEq)]
struct Vehicle {
    brand: String,
    model: String,
    year: i16,
    vehicle_type: VehicleType,
}

fn main() {
    let mut veh_vec: Vec<Vehicle> = Vec::new();

    let veh1 = Vehicle {
        brand: String::from("Toyota"),
        model: String::from("Corolla"),
        year: 2022,
        vehicle_type: VehicleType::Car(4),
    };

    let veh2 = Vehicle {
        brand: String::from("Yamaha"),
        model: String::from("R1"),
        year: 2021,
        vehicle_type: VehicleType::Motorcycle(false),
    };

    add_veh(veh1, &mut veh_vec);
    add_veh(veh2, &mut veh_vec);

    println!("Welcome to Simple Vehicle Management!");

    loop {
        println!("Press:\n 1 for Adding a vehicle\n 2 for Removing a vehicle\n 3 for Printing a List of all Vehicles\n 4 to Quit");
        let mut input = String::new();
        let valid_input = loop {
            input.clear();
            match io::stdin().read_line(&mut input) {
                Ok(_) => match input.trim().parse::<i32>() {
                    Ok(num) if (1..=4).contains(&num) => {
                        break num;
                    }
                    _ => {
                        println!("Invalid input. Please enter a number between 1 and 4");
                    }
                },
                Err(_) => {
                    println!("Failed to read line.");
                }
            }
        };

        match valid_input {
            1 => {
                let temp_veh = input_rigamarole();
                add_veh(temp_veh, &mut veh_vec);
                continue;
            }
            2 => {
                let temp_veh = input_rigamarole();
                remove_veh(temp_veh, &mut veh_vec);
                continue;
            }
            3 => {
                for vehicle in &veh_vec {
                    if let VehicleType::Car(doors) = &vehicle.vehicle_type {
                        println!(
                            "Vehicle: {} {} ({}) - Car with {} doors",
                            vehicle.brand, vehicle.model, vehicle.year, doors
                        );
                    }
                    if let VehicleType::Motorcycle(cab) = &vehicle.vehicle_type {
                        println!(
                            "Vehicle: {} {} ({}) - Motorcyle {}",
                            vehicle.brand,
                            vehicle.model,
                            vehicle.year,
                            if *cab {
                                "with a sidecar"
                            } else {
                                "without a sidecar"
                            }
                        );
                    }
                }
                continue;
            }
            4 => {
                println!("Exiting...");
                break;
            }
            _ => unreachable!(),
        }
    }
}
fn add_veh(vehicle: Vehicle, veh_vec: &mut Vec<Vehicle>) {
    veh_vec.push(vehicle);
}

fn remove_veh(vehicle: Vehicle, veh_vec: &mut Vec<Vehicle>) {
    veh_vec.retain(|v| v != &vehicle);
}

fn input_rigamarole() -> Vehicle {
    let mut brand_input = String::new();
    let mut model_input = String::new();
    let mut year_input = String::new();
    let mut vehicle_type_input = String::new();

    println!("Please enter the brand of the vehicle: ");
    io::stdin()
        .read_line(&mut brand_input)
        .expect("Invalid input");
    println!("Please enter the model of the vehicle: ");
    io::stdin()
        .read_line(&mut model_input)
        .expect("Invalid input");
    println!("Please enter the year of the vehicle: ");
    let valid_year = loop {
        io::stdin()
            .read_line(&mut year_input)
            .expect("Invalid Input");
        match year_input.trim().parse::<i16>() {
            Ok(num) => break num,
            _ => {
                println!("Invalid input. Please try again");
                continue;
            }
        }
    };
    println!("Is this vehicle a:\n 1: Motorcycle\n or 2: Car");
    let valid_vehicle_type = loop {
        vehicle_type_input.clear();
        io::stdin()
            .read_line(&mut vehicle_type_input)
            .expect("Invalid input");
        let choice: i8 = match vehicle_type_input.trim().parse::<i8>() {
            Ok(num) if (1..=2).contains(&num) => num,
            _ => {
                println!("Invalid input, please select 1 or 2.");
                continue;
            }
        };
        match choice {
            1 => {
                let cab = loop {
                    vehicle_type_input.clear();
                    println!("Please enter 0 for no Sidecar, or 1 for a Sidecar");
                    io::stdin()
                        .read_line(&mut vehicle_type_input)
                        .expect("Invalid input");
                    match vehicle_type_input.trim().parse::<i8>() {
                        Ok(1) => break true,
                        Ok(0) => break false,
                        _ => {
                            println!("Please enter 0 or 1");
                            continue;
                        }
                    }
                };
                break VehicleType::Motorcycle(cab);
            }
            _ => {
                let doors = loop {
                    vehicle_type_input.clear();
                    println!("Please enter the number of doors: ");
                    io::stdin()
                        .read_line(&mut vehicle_type_input)
                        .expect("Invalid input");
                    match vehicle_type_input.trim().parse::<i8>() {
                        Ok(num) if matches!(num, 2 | 4) => break num,
                        _ => {
                            println!(
                                "Theres no way a car has {} doors. Try again",
                                vehicle_type_input
                            );
                            continue;
                        }
                    }
                };
                break VehicleType::Car(doors);
            }
        }
    };

    let temp_veh = Vehicle {
        brand: brand_input.trim().to_string(),
        model: model_input.trim().to_string(),
        year: valid_year,
        vehicle_type: valid_vehicle_type,
    };

    return temp_veh;
}
