struct RepositorioNombres {
    nombres: Vec<String>,
}

impl RepositorioNombres {
    fn nuevo() -> Self {
        Self {
            nombres: Vec::new(),
        }
    }

    fn agregar(&mut self, n: String) {
        self.nombres.push(n);
    }

    fn listar(&self) -> &[String] {
        &self.nombres
    }
}

fn main() {
    let mut repo = RepositorioNombres::nuevo();
    repo.agregar("Ana".to_string());
    repo.agregar("Luis".to_string());
    println!("{:?}", repo.listar());
}
