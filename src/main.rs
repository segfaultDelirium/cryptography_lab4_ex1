use hex;

fn euclid_modulo(v: usize, n: usize) -> usize {
    let res = v % n;
    if res < 0 {res + n} else {res}
}


fn KSA(key: &String) -> Vec<i32>{
    let mut S = (0..=255).collect::<Vec<i32>>();
    let mut j: usize = 0;
    for i in (0..=255) {
        let key_index = euclid_modulo(i, key.len());
        let key_char = key.as_bytes()[key_index];
        j = euclid_modulo(j + S[i] as usize + key_char as usize, 256);
        (S[i], S[j]) = (S[j], S[i]);
        1 == 1;

    }

    S
}

fn stream_generator(plaintext: &String, input_S: Vec<i32>)-> (Vec<u8>, Vec<u8>){
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut S = input_S.clone();
    let mut result_ciphered: Vec<u8> = vec![];
    let mut key_stream: Vec<u8> = vec![];
    for el in plaintext.chars() {
        i = euclid_modulo(i + 1, 256);
        j = euclid_modulo((j + S[i] as usize), 256);
        (S[i], S[j]) = (S[j], S[i]);
        let s_index = euclid_modulo((S[i] + S[j]) as usize, 256);
        let byte_key = S[s_index] as u8;
        key_stream.push(byte_key);
        let to_push = el as u8  ^ byte_key;
        result_ciphered.push(to_push);
    }
    (result_ciphered, key_stream)
}

fn hex_vec_to_pretty_string(vector: Vec<u8>) -> String{
    vector.into_iter().map(|x| format!("{:02X}", x))
        .reduce(|acc, x| format!("{}{}", acc, x)).unwrap()

}

fn z1(){
    let plaintext = "Attack at dawn".to_string();
    let key = "Secret".to_string();
    let input_S = KSA(&key);

    // println!("keystream: {:?}", keystream);

    let (cyphertext, keystream) = stream_generator(&plaintext, input_S);


    let cyphertext_to_print = hex_vec_to_pretty_string(cyphertext);
    let keystream_to_print = hex_vec_to_pretty_string(keystream);

    println!("keystream: {}", keystream_to_print);
    println!("plaintext: {}", plaintext);
    println!("key: {}", &key);
    println!("cyphertext: {}", cyphertext_to_print);
}

fn z1_2(){
    let plaintext = "Plaintext".to_string();
    let key = "Key".to_string();
    let input_S = KSA(&key);

    // println!("keystream: {:?}", keystream);

    let (cyphertext, keystream) = stream_generator(&plaintext, input_S);


    let cyphertext_to_print = hex_vec_to_pretty_string(cyphertext);
    let keystream_to_print = hex_vec_to_pretty_string(keystream);

    println!("keystream: {}", keystream_to_print);
    println!("plaintext: {}", plaintext);
    println!("key: {}", &key);
    println!("cyphertext: {}", cyphertext_to_print);
}

fn z1_3(){
    let plaintext = "pedia".to_string();
    let key = "Wiki".to_string();
    let input_S = KSA(&key);

    // println!("keystream: {:?}", keystream);

    let (cyphertext, keystream) = stream_generator(&plaintext, input_S);


    let cyphertext_to_print = hex_vec_to_pretty_string(cyphertext);
    let keystream_to_print = hex_vec_to_pretty_string(keystream);

    println!("keystream: {}", keystream_to_print);
    println!("plaintext: {}", plaintext);
    println!("key: {}", &key);
    println!("cyphertext: {}", cyphertext_to_print);
}

fn main() {

    z1();
    println!();
    z1_2();
    println!();
    z1_3();
    println!();
    println!("bye");
}
