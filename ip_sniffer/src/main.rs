use std::env;
use std::net::{IpAddr, TcpStream};
use std::io::{self, Write};
use std::str::FromStr;
use std::process;
use std::sync::mpsc::{Sender, channel};
use std::thread;

const MAX: u16=65535;

struct Arguments {
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2{
            return Err("Parametros insuficientes");
        }else if args.len() > 4{
            return Err("Demasiados parametros (MAX: 4)");
        }
        let f = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&f){
            return Ok(Arguments {flag: String::from(""), ipaddr, threads: 4});
        }else{
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
                println!("Usa: -j para indiccar cuantos hilos quieres correr. \r\n -h o -help para mostrar un mensaje de ayuda");
                return Err("help");
            }else if flag.contains("-h") || flag.contains("-help") {
                return Err("muchos argumentos");
            }else if flag.contains("-j") {
                let ipaddr = match IpAddr::from_str(&args[3]){
                    Ok(s) => s,
                    Err(_) => return Err("IPADDR no es valido; debe ser IPv4 o IPv6")
                };
                let threads = match args[2].parse::<u16>(){
                    Ok(s) => s,
                    Err(_) => return Err("error pasando el numeroo de hilos")
                };
                return Ok(Arguments{threads, flag, ipaddr});
            }else{
                return Err("Error en sintaxis");
            }
        }
    }
}

fn scan(tx: Sender<u16>, start_port: u16, addr:IpAddr, num_threads: u16){
    let mut port: u16 = start_port + 1;
    loop{
        match TcpStream::connect((addr, port)){
            Ok(_)=>{
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) =>{}
        }
        if (MAX - port) <= num_threads {
            break;
        }
        port += num_threads;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(
        |err| {
            if err.contains("help") {
                process::exit(0);
            }else{
                eprintln!("{} prroblems: {}", program, err);
                process::exit(0);
            }
        }
    );

    let num_threads = arguments.threads;
    let addr = arguments.ipaddr;
    let (tx, rx) = channel();
    for i in 0 .. num_threads{
        let tx = tx.clone();

        thread::spawn(move || {
            scan(tx, i, addr, num_threads);
        });
    }

    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }

    println!("");
    out.sort();
    for v in out {
        println!("{} esta abierto ", v);
        if(v.contains("45")){
            println!("{} SAMBA ", v);
            for i in 0 .. num_threads{
                //lanzar package
            }
        }
    }
}
