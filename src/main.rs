use std::io::{self, Write};

fn main() {
    loop {
        println!("\nKonversi Suhu (Fahrenheit <-> Celsius)");
        println!("1. Konversi Celsius ke Fahrenheit");
        println!("2. Konversi Fahrenheit ke Celsius");
        println!("3. Keluar");
        print!("Pilih opsi (1/2/3): ");
        io::stdout().flush().expect("Gagal meng-*flush* stdout");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Gagal membaca input.");

        match choice.trim() {
            "1" => celsius_to_fahrenheit(),
            "2" => fahrenheit_to_celsius(),
            "3" => {
                println!("Keluar dari program. Sampai jumpa!");
                break;
            }
            _ => println!("Pilihan tidak valid, coba lagi!"),
        }
    }
}

fn celsius_to_fahrenheit() {
    print!("Masukkan suhu dalam Celsius: ");
    io::stdout().flush().expect("Gagal meng-*flush* stdout");

    let celsius = read_input();
    let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
    println!("{celsius}°C sama dengan {fahrenheit}°F");
}

fn fahrenheit_to_celsius() {
    print!("Masukkan suhu dalam Fahrenheit: ");
    io::stdout().flush().expect("Gagal meng-*flush* stdout");
    
    let fahrenheit = read_input();
    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
    println!("{fahrenheit}°F sama dengan {celsius}°C");
}

fn read_input() -> f64 {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Gagal membaca input.");

        match input.trim().parse::<f64>() {
            Ok(num) => return num, // Jika valid, langsung kembalikan angka
            Err(_) => {
                print!("Input tidak valid, masukkan angka lagi: "); // Minta ulang
                io::stdout().flush().expect("Gagal meng-*flush* stdout");
            }
        }
    }
}