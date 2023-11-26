# One instruction CPU

Special properties:
 * No opcode, only args
 * Args: DSTaddr SRCaddr JMPaddr(relative)

How it works?
 * mem[DSTaddr] -= mem[SRCaddr];
 * if mem[DSTaddr] != 0 { PC += JMPaddr }

All other function can create as a memory mapped function.
