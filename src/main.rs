// use chrono for get current date
use chrono::{Local};
fn main() {

    /* 
    declare variable 
    variable is where we store data
    */
    let nama_barang : &str = "Sikat gigi";
    let jumlah :i32 = 5;
    let diskon :f32 = 0.15;
    let diskon_convert :f32  = diskon * 100.0;
    let diskon_persen :u32 = diskon_convert as u32;  
    let kartu_member :bool = false;
    const FIRST_LATTER_NAME :char = 'L'; //biasakan untuk declare variable constant dengan capital letter

    // library
    let dt  = Local::now();
    let date  = dt.format("%Y-%m-%d").to_string();
    // %Y-%m-%d %H:%M:%S => 2022-12-27 05:45:57

    println!("Sekarang tanggal [{}]", date);
    println!("Library with crono");

    print!("Nama pelanggan, Leindaf ");
    println!("Dengan huruf depannya, {FIRST_LATTER_NAME}");

    println!("Nama barang {0}; \nJumlah Barang {1};", nama_barang, jumlah);
    println!("Mempunyai kartu member [{}]", !kartu_member);
    println!("Jumlah barang lebih dari 4 [{}]", (jumlah >= 4));
    if !kartu_member && jumlah > 4 {
        println!("Mendapatkan diskon sebesar, {} atau {}%", diskon, diskon_persen);
    } else {
        println!("Kamu tidak mendapat diskon");
    }

    println!("End of program...");
}   
