use objc::{
    class,
    declare::ClassDecl,
    msg_send,
    runtime::{Object, Sel},
    sel, sel_impl,
};

extern "C" fn rfcomm_closed(_: &Object, _: Sel) {
    println!("RFCOMM channel closed");
}

extern "C" fn rfcomm_changed(_: &Object, _: Sel) {
    println!("RFCOMM channel signal changed");
}

extern "C" fn rfcomm_data(_: &Object, _: Sel, _: *mut std::ffi::c_void, _: usize) {
    println!("RFCOMM receive data");
}

extern "C" fn rfcomm_flow(_: &Object, _: Sel) {
    println!("RFCOMM flow control changed");
}

extern "C" fn rfcomm_open_complete(_: &Object, _: Sel, _: isize) {
    println!("RFCOMM open complete");
}

extern "C" fn rfcomm_queue_space(_: &Object, _: Sel) {
    println!("RFCOMM queue space");
}

extern "C" fn rfcomm_write_complete(_: &Object, _: Sel, _: *mut std::ffi::c_void) {
    println!("Write complete")
}

extern "C" fn l2cap_closed(_: &Object, _: Sel) {
    println!("L2CAP channel closed");
}

extern "C" fn l2cap_data(_: &Object, _: Sel, _: *mut std::ffi::c_void, _: usize) {
    println!("Data avaiablle on l2cap channel");
}

extern "C" fn l2cap_open(_: &Object, _: Sel, status: isize) {
    println!("l2cap channel open");
}

extern "C" fn l2cap_space(_: &Object, _: Sel) {
    println!("Space available");
}

extern "C" fn l2cap_reconfigured(_: &Object, _: Sel) {
    println!("Channel reconfigured");
}

extern "C" fn l2cap_write_complete(_: &Object, _: Sel, _: *mut std::ffi::c_void, _: isize) {
    println!("l2cap write completed");
}

fn main() {
    unsafe {
        let superclass = class!(NSObject);
        let mut decl = ClassDecl::new("RFCOMMDelegate", superclass).unwrap();
        decl.add_method(
            sel!(rfCommChannelClosed:),
            rfcomm_closed as extern "C" fn(&Object, Sel),
        );
        decl.add_method(
            sel!(rfcommChannelControlSignalsChanged:),
            rfcomm_changed as extern "C" fn(&Object, Sel),
        );
        decl.add_method(
            sel!(rfcommChannelData:data:length:),
            rfcomm_data as extern "C" fn(&Object, Sel, *mut std::ffi::c_void, usize),
        );
        decl.add_method(
            sel!(rfcommChannelFlowControlChanged:),
            rfcomm_flow as extern "C" fn(&Object, Sel),
        );
        decl.add_method(
            sel!(rfcommChannelOpenComplete:status:),
            rfcomm_open_complete as extern "C" fn(&Object, Sel, isize),
        );
        decl.add_method(
            sel!(rfcommChannelQueueSpaceAvailable:),
            rfcomm_queue_space as extern "C" fn(&Object, Sel),
        );
        decl.add_method(
            sel!(rfcommChannelWriteComplete:refcon:),
            rfcomm_write_complete as extern "C" fn(&Object, Sel, *mut std::ffi::c_void),
        );
        let class = decl.register();
        let rfcomm_delegate: *mut Object = msg_send![class, new];
        let mut l2cap_decl = ClassDecl::new("L2CAPDelegate", superclass).unwrap();
        l2cap_decl.add_method(
            sel!(l2capChannelClosed:),
            l2cap_closed as extern "C" fn(&Object, Sel),
        );
        l2cap_decl.add_method(
            sel!(l2capChannelData:data:length:),
            l2cap_data as extern "C" fn(&Object, Sel, *mut std::ffi::c_void, usize),
        );
        l2cap_decl.add_method(
            sel!(l2capChannelOpenComplete:status:),
            l2cap_open as extern "C" fn(&Object, Sel, isize),
        );
        l2cap_decl.add_method(
            sel!(l2capChannelQueueSpaceAvailable:),
            l2cap_space as extern "C" fn(&Object, Sel),
        );
        l2cap_decl.add_method(
            sel!(l2capChannelReconfigured:),
            l2cap_reconfigured as extern "C" fn(&Object, Sel),
        );
        l2cap_decl.add_method(
            sel!(l2capChannelWriteComplete:refcon:status:),
            l2cap_write_complete as extern "C" fn(&Object, Sel, *mut std::ffi::c_void, isize),
        );
        let l2cap_class = l2cap_decl.register();
        let l2cap_delegate: *mut Object = msg_send![l2cap_class, new];
    }
    println!("Hello, world!");
}
