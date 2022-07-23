use systemstat::Platform;

fn main() {
    let sys_stat = systemstat::System::new();
    do_something(&sys_stat);
}

fn do_something(sys_stat: &systemstat::System) {
    let net = sys_stat.networks(); // crash (exit code: 0xc0000005, STATUS_ACCESS_VIOLATION)
}
