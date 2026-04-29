use std::io;

fn calculate_cosine_similarity() {
    // ---------------------------------------------------------
    // 1. VEKTÖRÜ OKUMA
    // ---------------------------------------------------------
    println!("Write the first vector's coordinates (x y z):");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Satır okunamadı");
    
    let mut parts1 = input1.split_whitespace();
    // f64 kullanarak yüksek hassasiyetli ondalıklı sayılara çeviriyoruz
    let x: f64 = parts1.next().unwrap().parse().unwrap();
    let y: f64 = parts1.next().unwrap().parse().unwrap();
    let z: f64 = parts1.next().unwrap().parse().unwrap();

    // ---------------------------------------------------------
    // 2. VEKTÖRÜ OKUMA
    // ---------------------------------------------------------
    println!("Write the second vector's coordinates (x y z):");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Satır okunamadı");
    
    let mut parts2 = input2.split_whitespace();
    let x2: f64 = parts2.next().unwrap().parse().unwrap();
    let y2: f64 = parts2.next().unwrap().parse().unwrap();
    let z2: f64 = parts2.next().unwrap().parse().unwrap();

    // ---------------------------------------------------------
    // HESAPLAMA ADIMLARI
    // ---------------------------------------------------------
    // 1. Skaler Çarpım (Nokta Çarpım)
    let dot_product = (x * x2) + (y * y2) + (z * z2);
    
    // 2. Uzunlukların Bulunması (Kütüphanesiz, direkt .sqrt() ile)
    let vector1_length = (x * x + y * y + z * z).sqrt();
    let vector2_length = (x2 * x2 + y2 * y2 + z2 * z2).sqrt();
    
    // 3. Sonuç (Bölme işlemi)
    let result_score = dot_product / (vector1_length * vector2_length);

    // Ekrana yazdırma
    println!("Cosine Similarity: {}", result_score);
}

fn main() {
    calculate_cosine_similarity();
}