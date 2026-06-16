pub fn validar_cpf(cpf: &str) -> bool {
    // Remove caracteres que não são números
    let cpf: String = cpf.chars().filter(|c| c.is_ascii_digit()).collect();

    // CPF deve ter 11 dígitos
    if cpf.len() != 11 {
        return false;
    }

    let numeros: Vec<u32> = cpf
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    // Rejeita CPFs com todos os dígitos iguais
    if numeros.iter().all(|&n| n == numeros[0]) {
        return false;
    }

    // Primeiro dígito verificador
    let soma1: u32 = numeros[..9]
        .iter()
        .enumerate()
        .map(|(i, &n)| n * (10 - i as u32))
        .sum();

    let resto1 = soma1 % 11;
    let dv1 = if resto1 < 2 { 0 } else { 11 - resto1 };

    if numeros[9] != dv1 {
        return false;
    }

    // Segundo dígito verificador
    let soma2: u32 = numeros[..10]
        .iter()
        .enumerate()
        .map(|(i, &n)| n * (11 - i as u32))
        .sum();

    let resto2 = soma2 % 11;
    let dv2 = if resto2 < 2 { 0 } else { 11 - resto2 };

    numeros[10] == dv2
}
