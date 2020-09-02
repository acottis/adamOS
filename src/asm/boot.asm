global start
extern kmain

section .text
bits 32
start:

create_pages:
         ; Point the first entry of the level 4 page table to the first entry in the
        ; p3 table
        mov eax, p3_table
        or eax, 0b11
        mov dword [p4_table + 0], eax

        ; Point the first entry of the level 3 page table to the first entry in the
        ; p2 table
        mov eax, p2_table
        or eax, 0b11
        mov dword [p3_table + 0], eax

        ; Point each page table level two entry to a page
        mov ecx, 0 ; Counter variable

.map_p2_table:
        mov eax, 0x200000 ; 2MiB
        mul ecx ; multiply 2MiB by the current counter 
        or eax, 0b10000011 ; Or set the binary, since it is zeroed before, it will look like 10000011
        mov [p2_table + ecx * 8], eax ; move the 8 bits in eax into p2_table at the counter*8 location
        inc ecx ; increment counter 
        cmp ecx, 512 ; are we at 512?
        jne .map_p2_table ; if we are at 512 stop

; https://en.wikipedia.org/wiki/Control_register 
enable_paging:
        ; move page table address to cr3, cr3 is used to become the initial page location that references the others
        mov eax, p4_table
        mov cr3, eax

        ; enable Physical Address Extension PAE
        mov eax, cr4
        or eax, 1 << 5 ; 0b100000
        mov cr4, eax

        ; set the long mode bit
        mov ecx, 0xC0000080
        rdmsr
        or eax, 1 << 8; 0b100000000
        wrmsr

        ; enable paging 
        mov eax, cr0
        or eax, 1 << 31 ; Sets 31st bit to 1 to enable paging
        or eax, 1 << 16 ; Sets 16th bit to 1 to prevent CPU writing to read-only pages
        mov cr0, eax

finish_setup:
        ; tell hardware about our GDT
        lgdt [gdt64.pointer]

        ; update selectors
        mov ax, gdt64.data
        mov ss, ax
        mov ds, ax
        mov es, ax

        ; jump to long mode and rust entry point!
        jmp gdt64.code:kmain

helloworld32:
        mov word [0xb8000], 0x0248 ; H
        mov word [0xb8002], 0x0265 ; e
        mov word [0xb8004], 0x026c ; l
        mov word [0xb8006], 0x026c ; l
        mov word [0xb8008], 0x026f ; o
        mov word [0xb800a], 0x022c ; ,
        mov word [0xb800c], 0x0220 ;
        mov word [0xb800e], 0x0277 ; w
        mov word [0xb8010], 0x026f ; o
        mov word [0xb8012], 0x0272 ; r
        mov word [0xb8014], 0x026c ; l
        mov word [0xb8016], 0x0264 ; d
        mov word [0xb8018], 0x0221 ; !
        hlt

; Reserves bytes for page tables
section .bss
align 4096
p4_table: resb 4096
p3_table: resb 4096
p2_table: resb 4096

; Sets up segmentation as a hack to get long mode
section .rodata
gdt64:
        dq 0
;  44: ‘descriptor type’: This has to be 1 for code and data segments
;  47: ‘present’: This is set to 1 if the entry is valid
;  41: ‘read/write’: If this is a code segment, 1 means that it’s readable
;  43: ‘executable’: Set to 1 for code segments
;  53: ‘64-bit’: if this is a 64-bit GDT, this should be set
.code: equ $ - gdt64
        dq (1<<44) | (1<<47) | (1<<41) | (1<<43) | (1<<53)
.data: equ $ - gdt64
        dq (1<<44) | (1<<47) | (1<<41)
.pointer:
        dw .pointer - gdt64 -1
        dq gdt64

section .text
bits 64
long_mode_start:

        ;mov rax, 0x2f592f412f4b2f4f
        ;mov qword [0xb8000], rax

       ; hlt