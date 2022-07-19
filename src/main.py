from sisprog import CPUState, cycle, read_memory, write_many, write_memory, get_acc, get_c, get_la, get_n, get_p, get_pc, get_print, feed_read, get_saved_reg, get_sp, get_state,  get_v, get_z, parse_binary, execute

def sign_extend(num: int) -> int:
    if num < 0:
        return num
    
    sign_bit = 1 << 31
    return (num & (sign_bit - 1)) - (num & sign_bit) 

if __name__ == "__main__":

    ndata, ninstr, data, instr = parse_binary("../ex.fita")

    write_many(
        0x0,
        instr
    )

    write_many(
        0x10000,
        data
    )

    # processor.write_many(
    #     0x10000,
    #     data
    # )

    # ndata, ninstr, data, instr = parse_binary("ex.fita")

    # processor.write_many(
    #     0x30000,
    #     data + instr
    # )

    # processor.write_many(
    #     0x20000, [
    #         11,
    #         50,
    #         0x60000,
    #         ninstr,
    #         ndata,
    #         0x20000 + 11,
    #         50,
    #     ]
    # )

    execute(0x0, True)
    while True:
        cycle()

        print(get_state())

        if get_state() == CPUState.IDLE:
            break

        if get_state() == CPUState.INPUT:
            a = input("")
            feed_read(int.from_bytes(a.encode('utf-8'), 'little'))

        if get_state() == CPUState.OUTPUT:
            the_string = get_print()
            print(bytes(the_string).decode('utf-8'))

    
    
    # for i in range(0x20000, 0x2000f):
    #     print(f"{i:x}: {processor.read_memory(i)}")

    # for i in range(0x20000000, 0x20000030):
    #     print(f"{i:x}: {processor.read_memory(i)}")

    # print(processor.read_memory(0x40))
    # print("ee")
    # processor.pc = 0x40

    # processor.state = CPUState.STEP
    # while True:
    #     processor.cycle()
    #     print(f"State:{processor.state}\nACC: {sign_extend(processor.acc)}\nPC: {processor.pc}\nLA: {processor.la}\nIZNCV: {processor.i} {processor.z} {processor.n} {processor.c} {processor.v}\n")
        
    #     if processor.state == CPUState.IDLE:
    #         break

    #     if processor.state == CPUState.INPUT:
    #         a = input("")
    #         processor.feed_read(int.from_bytes(a.encode('utf-8'), 'little'))

    #     if processor.state == CPUState.OUTPUT:
    #         the_string = processor.get_print()
    #         print(bytes(the_string).decode('utf-8'))