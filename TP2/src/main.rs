use std::{fs::File, io::Read};
// o collections serve para importar bibliotecas de coleções de dados
// eh melhor usar o vector em vez de array nessa tarefa
// o porque o usar o vecto, quando vc n quer usar o array statico
// o vector ele `` meio `` que ja vai alocar na heap
mod pokemon;
use pokemon::Pokemon;

fn main() {
    let path = "C:\\Users\\joao_\\Downloads\\pokemon.csv";
    let mut file = File::open(path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("nao achei essa merda");

    //essa variavel vai transformar o buffer em string todo buffer
    let csv = String::from_utf8(buffer).unwrap();

    //array de ponteiros para pokemons
    let mut pokemons: Vec<Box<Pokemon>> = Vec::with_capacity(400);

    //criando um interador que vai pegar as linhas do csv
    let mut linhas = csv.lines();

    //pulando a primeira linha
    linhas.next();
    for linha in linhas {
        pokemons.push(Box::new(Pokemon::from_str(linha.to_string())));
    }
    for pokemon in pokemons {
        println!("{}", pokemon);
    }
}
