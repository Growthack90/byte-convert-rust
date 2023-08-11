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

        // Richiedi all'utente di inserire l'unità di misura
        println!("Seleziona l'unità di misura (B, KB, MB, GB, TB):");
        let mut input_unit = String::new();
        io::stdin().read_line(&mut input_unit).expect("Errore nella lettura dell'input");

        // Effettua la conversione in base all'unità di misura scelta
        let result = match input_unit.trim().to_lowercase().as_str() {
            "b" => input_value,
            "kb" => input_value * 1024.0,
            "mb" => input_value * 1024.0 * 1024.0,
            "gb" => input_value * 1024.0 * 1024.0 * 1024.0,
            "tb" => input_value * 1024.0 * 1024.0 * 1024.0 * 1024.0,
            _ => {
                println!("Unità di misura non valida. Riprova.");
                continue; // Torna all'inizio del ciclo
            }
        };

        // Stampa il risultato
        println!("Risultato {} --> B: {:.2} B", input_unit.trim(), result);


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

