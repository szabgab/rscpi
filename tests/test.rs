mod io;
use rscpi::*;

#[test]
fn main() {
    //let last_btag: u8 = 0x00;

    let message: String = String::from("Hello fellow Rustaceans!");

    println!("{}", message);

    //let device_all = nusb::list_devices().unwrap();
    //println!("{:#?}", device_all.collect::<Vec<_>>()); // Collect the iterator into a Vec and print it with pretty formatting

    let device_info: nusb::DeviceInfo = nusb::list_devices()
        .unwrap()
        .find(|dev| dev.vendor_id() == 0x2A8D && dev.product_id() == 0x0397)
        .expect("device not connected");

    let device: nusb::Device = device_info.open().expect("failed to open device");
    let interface: nusb::Interface = device.detach_and_claim_interface(0).unwrap();

    //let config: nusb::descriptors::Configuration<'_> = device.active_configuration().unwrap();

    //println!("Active configuration: {:#?}", config);

    let mut usbtmc = Usbtmc {
        interface,
        recv_buffer_size: 1024,
    };

    let idn = query(&mut usbtmc, "*IDN?").unwrap();
    print!("{}", idn);

    println!("{}", query(&mut usbtmc, ":CHANnel1:SCALe?").unwrap());

    write(&mut usbtmc, ":TIMebase:MODE MAIN").unwrap();

    write(&mut usbtmc, ":WAVeform:POINts:MODE RAW").unwrap();

    write(&mut usbtmc, ":DIGitize CHANnel1").unwrap();

    write(&mut usbtmc, ":WAVeform:FORMat BYTE").unwrap();

    write(&mut usbtmc, ":WAVeform:POINts 10151").unwrap();

    let data = query_raw(&mut usbtmc, ":WAVeform:DATA?").unwrap();

    //let _data = send_command(&mut usbtmc, ":DISP:DATA? BMP, COL").unwrap();

    //let data = send_command_raw(&mut usbtmc, ":DISP:DATA? PNG, COL").unwrap();

    let sliced_data = &data[10..data.len() - 1];

    io::write_to_file(sliced_data, "./output/output.png").expect("failed to write to file");
}
