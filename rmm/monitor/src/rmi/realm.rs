use crate::event::Mainloop;
use crate::listen;
use crate::rmi;

extern crate alloc;

pub fn set_event_handler(mainloop: &mut Mainloop) {
    listen!(mainloop, rmi::Code::RealmCreate, |ctx, rmm, _| {
        let ret = rmm.create();
        match ret {
            Ok(id) => {
                ctx.ret[0] = rmi::RET_SUCCESS;
                ctx.ret[1] = id;
            }
            Err(_) => ctx.ret[0] = rmi::RET_FAIL,
        }
    });

    listen!(mainloop, rmi::Code::VCPUCreate, |ctx, rmm, _| {
        let id = ctx.arg[0];
        let ret = rmm.create_vcpu(id);
        match ret {
            Ok(vcpuid) => {
                ctx.ret[0] = rmi::RET_SUCCESS;
                ctx.ret[1] = vcpuid;
            }
            Err(_) => ctx.ret[0] = rmi::RET_FAIL,
        }
    });

    listen!(mainloop, rmi::Code::RealmDestroy, |ctx, rmm, _| {
        let id = ctx.arg[0];
        let ret = rmm.remove(id);
        match ret {
            Ok(_) => ctx.ret[0] = rmi::RET_SUCCESS,
            Err(_) => ctx.ret[0] = rmi::RET_FAIL,
        }
    });

    listen!(mainloop, rmi::Code::RealmRun, |ctx, rmm, _| {
        let id = ctx.arg[0];
        let vcpu = ctx.arg[1];
        let incr_pc = ctx.arg[2];
        let ret = rmm.run(id, vcpu, incr_pc);
        match ret {
            Ok(val) => match val[0] {
                rmi::RET_EXCEPTION_TRAP | rmi::RET_EXCEPTION_IRQ => {
                    ctx.ret = [val[0], val[1], val[2], val[3], 0, 0, 0, 0];
                }
                _ => ctx.ret[0] = rmi::RET_SUCCESS,
            },
            Err(_) => ctx.ret[0] = rmi::RET_FAIL,
        };
    });

    listen!(mainloop, rmi::Code::RealmMapMemory, |ctx, rmm, _| {
        let id = ctx.arg[0];
        let guest = ctx.arg[1];
        let phys = ctx.arg[2];
        let size = ctx.arg[3];
        // prot: bits[0] : writable, bits[1] : fault on exec, bits[2] : device
        let prot = ctx.arg[4]; // bits[3]
        let ret = rmm.map(id, guest, phys, size, prot);
        match ret {
            Ok(_) => ctx.ret[0] = rmi::RET_SUCCESS,
            Err(_) => ctx.ret[0] = rmi::RET_FAIL,
        }
    });

    listen!(mainloop, rmi::Code::RealmUnmapMemory, |ctx, rmm, _| {
        let id = ctx.arg[0];
        let guest = ctx.arg[1];
        let size = ctx.arg[2];
        let ret = rmm.unmap(id, guest, size);
        match ret {
            Ok(_) => ctx.ret[0] = rmi::RET_SUCCESS,
            Err(_) => ctx.ret[0] = rmi::RET_FAIL,
        }
    });

    listen!(mainloop, rmi::Code::RealmSetReg, |ctx, rmm, _| {
        let id = ctx.arg[0];
        let vcpu = ctx.arg[1];
        let register = ctx.arg[2];
        let value = ctx.arg[3];
        let ret = rmm.set_reg(id, vcpu, register, value);
        match ret {
            Ok(_) => ctx.ret[0] = rmi::RET_SUCCESS,
            Err(_) => ctx.ret[0] = rmi::RET_FAIL,
        }
    });

    listen!(mainloop, rmi::Code::RealmGetReg, |ctx, rmm, _| {
        let id = ctx.arg[0];
        let vcpu = ctx.arg[1];
        let register = ctx.arg[2];
        let ret = rmm.get_reg(id, vcpu, register);
        match ret {
            Ok(_) => ctx.ret[0] = rmi::RET_SUCCESS,
            Err(_) => ctx.ret[0] = rmi::RET_FAIL,
        }
    });
}
