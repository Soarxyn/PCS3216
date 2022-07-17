from sisprog import CPU, CPUState, print_debug

def sign_extend(num: int) -> int:
    if num < 0:
        return num
    
    sign_bit = 1 << 31
    return (num & (sign_bit - 1)) - (num & sign_bit) 

if __name__ == "__main__":
    processor = CPU()

    processor.write_many(
        0x400, [
            0x1,
            0x402,
            0x0
        ]
    )

    processor.write_many(
        0x0, [
            0x08000400, # LDA 400
            0x18000400, # ADD 400
            0x06000010, # SET 10000
            0x10000401, # STA 402i
        ]
    )
    # processor.write_many(0x400, [0, 1, 2])
    # processor.write_many(0x0, [
    #         0x08000404, # 0  LDA 404
    #         0x18000408, # 4  ADD 408
    #         0x1000040C, # 8  STA 40C
    #         0x20000404, # 12 SUB 404
    #         0x2800040C, # 16 MUL 40C
    #         0x30000408, # 20 DIV 408
    #         0x3800040C, # 24 CMP 40C
    #         0x48000020, # 28 BEQ 20 
    #         0x40000000, # 32 NEG
    #         0x18000404, # 36 ADD 404
    #         0xC8000001, # 40 LSR 1
    #         0xC0000002, # 44 LSL 2
    #         0x90000038, # 48 JAL 38
    #         0x00000000, # 52 HALT
    #         0x04000410, # 56 READ 410
    #         0x08000410, # 60 LDA 410
    #         0x18000410, # 64 ADD 404
    #         0xF8000000, # 68 RET
    #     ]
    # )

    processor.state = CPUState.STEP

    while True:
        processor.cycle()
        print(f"ACC: {sign_extend(processor.acc)}\nPC: {processor.pc}\nLA: {processor.la}\nIZNCV: {processor.i} {processor.z} {processor.n} {processor.c} {processor.v}\n")

        if processor.state == CPUState.IDLE:
            break

        if processor.state == CPUState.INPUT:
            a = input("")
            processor.feed_read(int.from_bytes(a.encode('utf-8'), 'little'))

        if processor.state == CPUState.OUTPUT:
            the_string = processor.get_print()
            print(bytes(the_string).decode('utf-8'))

    print(processor.read_memory(0x402))
#     assemblyResult = assemble("ex.qck", "ex.bdc")[1]
#     if assemblyResult == "Assembly successful":
#         linkingResult = link(["ex.bdc"], "ex.fita")[1]
#         if linkingResult == "Linking successful":
#             print_debug("ex.fita")
#         else:
#             print(linkingResult)
#     else:
#         print(assemblyResult)
