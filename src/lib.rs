

fn open_bus(iface_name: &str) {
    let interfaces = pnet::datalink::interfaces();
    let _interface = interfaces
        .into_iter()
        .find(|iface| iface.name == iface_name)
        .unwrap();

    println!("Managed to get here I think");

    
}


// The tests require a dummy interface if they're not run with a can module
// sudo ip link add name can0 type dummy
// make sure dummy is loaded, otherwise sudo modprobe dummy


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        crate::open_bus("can0");
        println!("finished it works");
        assert_eq!(5, 5);
    }
}