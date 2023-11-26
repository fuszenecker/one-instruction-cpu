# One instruction CPU

Special properties:
 * No opcode, only args
 * Args: addrA addrB JMPaddr(relative)
 * memory addresses is u8, data and JMPaddr are i16.
 * half of addressed memory is RAM, second half is preloaded "ROM" area. 
 * some memory address has a special function, e.g. addr-0 is INPUT/OUTPUT.

How it works?
 * mem[addrA] -= mem[addrB];
 * if mem[addrA] <= 0 { PC += JMPaddr }

All other function can create as a memory mapped function.
