// 0,1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, ...
use clap::Parser;

#[derive(Parser)]
#[clap(
    name = "fibo",
    author = "Alaf do Nascimento Santos",
    about = "Compute Fibonacci suite values"
)]
struct Args {
    /// The maximal number to print the fibo value of
    value: u32,

    #[clap(short, long)]
    /// Print intermediate values
    verbose: bool,

    #[clap(short = 'm', long = "min", default_value = "0", value_name = "NUMBER")]
    /// The minimum number to compute
    min: u32,
}

// Implémentation récursive de la fonction fibo
// fn fibo(n: u32) -> u32 {
//     match n {
//         0 => 0,
//         1 | 2 => 1,
//         _ => fibo(n - 1) + fibo(n - 2),
//     }
// }

// Implémentation itérative de la fonction fibo
// fn fibo(n: u32) -> u32 {
//     if n == 0 {
//         return 0;
//     } else if n < 2 {
//         return 1;
//     } else {
//         let mut somme = 0;
//         let mut valeur_avant = 0;
//         let mut valeur_maintenant = 1;
//         for _ in 1..n {
//             // somme = valeur_avant + valeur_maintenant;
//             // somme = u32::saturating_add(valeur_avant, valeur_maintenant);
//             somme = u32::checked_add(valeur_avant, valeur_maintenant).unwrap();
//             valeur_avant = valeur_maintenant;
//             valeur_maintenant = somme;
//         }
//         return somme;
//     }
// }

// Affichage des valeurs correctes uniquement
fn fibo(n: u32) -> Option<u32> {
    if n == 0 {
        return Some(0);
    } else if n < 2 {
        return Some(1);
    } else {
        let mut somme = 0;
        let mut valeur_avant = 0;
        let mut valeur_maintenant = 1;
        for _ in 1..n {
            match u32::checked_add(valeur_avant, valeur_maintenant) {
                Some(val) => somme = val,
                None => return None,
            }
            valeur_avant = valeur_maintenant;
            valeur_maintenant = somme;
        }
        return Some(somme);
    }
}

fn main() {
    let args = Args::parse();
    let position = args.min; // debut (0 est le default)
    let borne_maximale = args.value + 1; // + 1 pour faire if(position <= borne_maximale)

    for position in position..borne_maximale {
        if let Some(result) = fibo(position) {
            // Afficher les valeurs intermédiaires
            if args.verbose {
                println!("fibo({position}) = {:?}", result);
            }
            // Afficher uniquement la première et la dernière valeur
            else if position == args.min || position == args.value {
                println!("fibo({position}) = {:?}", result);
            }
        } else {
            break;
        }
    }
}
