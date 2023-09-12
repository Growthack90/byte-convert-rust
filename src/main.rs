use std::io;

fn main() {
    println!("Benvenuto nel convertitore di unità di memoria!");

    loop {
        // Richiedi all'utente di inserire il valore numerico
        println!("Inserisci il valore numerico che vuoi convertire in Byte:");
        let mut input_value = String::new();
        io::stdin().read_line(&mut input_value).expect("Errore nella lettura dell'input");

        // Converti l'input in un numero a virgola mobile (f64)
        let input_value: f64 = match input_value.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Valore non valido. Riprova.");
                continue; // Torna all'inizio del ciclo
            }
        };

        // Richiedi all'utente di inserire l'unità di misura di input
        println!("Seleziona l'unità di misura di input (B, KB, MB, GB, TB):");
        let mut input_unit = String::new();
        io::stdin().read_line(&mut input_unit).expect("Errore nella lettura dell'input");

        // Richiedi all'utente di inserire l'unità di misura di output
        println!("Seleziona l'unità di misura di output (B, KB, MB, GB, TB):");
        let mut output_unit = String::new();
        io::stdin().read_line(&mut output_unit).expect("Errore nella lettura dell'input");

        // Effettua la conversione in base all'unità di misura scelta
        let result = match (input_unit.trim().to_lowercase().as_str(), output_unit.trim().to_lowercase().as_str()) {
            ("b", "b") => input_value,
            ("kb", "b") => input_value * 1024.0,
            ("mb", "b") => input_value * 1024.0 * 1024.0,
            ("gb", "b") => input_value * 1024.0 * 1024.0 * 1024.0,
            ("tb", "b") => input_value * 1024.0 * 1024.0 * 1024.0 * 1024.0,
            ("b", "kb") => input_value / 1024.0,
            ("kb", "kb") => input_value,
            ("mb", "kb") => input_value * 1024.0,
            ("gb", "kb") => input_value * 1024.0 * 1024.0,
            ("tb", "kb") => input_value * 1024.0 * 1024.0 * 1024.0,
            ("b", "mb") => input_value / (1024.0 * 1024.0),
            ("kb", "mb") => input_value / 1024.0,
            ("mb", "mb") => input_value,
            ("gb", "mb") => input_value * 1024.0,
            ("tb", "mb") => input_value * 1024.0 * 1024.0,
            ("b", "gb") => input_value / (1024.0 * 1024.0 * 1024.0),
            ("kb", "gb") => input_value / (1024.0 * 1024.0),
            ("mb", "gb") => input_value / 1024.0,
            ("gb", "gb") => input_value,
            ("tb", "gb") => input_value * 1024.0,
            ("b", "tb") => input_value / (1024.0 * 1024.0 * 1024.0 * 1024.0),
            ("kb", "tb") => input_value / (1024.0 * 1024.0 * 1024.0),
            ("mb", "tb") => input_value / (1024.0 * 1024.0),
            ("gb", "tb") => input_value / 1024.0,
            ("tb", "tb") => input_value,
            _ => {
                println!("Unità di misura non valida. Riprova.");
                continue; // Torna all'inizio del ciclo
            }
        };

        // Stampa il risultato
        print!("Risultato:\n {} ({}) --> {:.8} ({})\n", input_value, input_unit.trim(), result, output_unit.trim());


        // Chiedi all'utente se desidera eseguire un'altra conversione
        println!("Vuoi eseguire un'altra conversione? (s/n)");
        let mut continue_input = String::new();
        io::stdin().read_line(&mut continue_input).expect("Errore nella lettura dell'input");
        
        if continue_input.trim().to_lowercase() != "s" {
            println!("Arrivederci!");
            break; // Esci dal ciclo
        }
    }
}

