format pe64 dll efi
entry main

section '.text' code readable executable

main:
    mov [ImageHandle], rcx
    mov [SystemTable], rdx

    mov rax, [rdx + 96]           ; SystemTable->BootServices
    mov [BootServices], rax

    lea rsi, [kernel_start]
    mov rdi, 0x100000             ; kernel addr 
	mov ecx, kernel_size          ; bounds
    cld
    rep movsb

    mov rax, [BootServices]
    lea rcx, [MemMapSize]
    lea rdx, [MemMap]
    lea r8,  [MapKey]
    lea r9,  [DescriptorSize]
    lea r10, [DescriptorVersion]
    sub rsp, 40
    mov [rsp + 32], r10           	
	call qword [rax + 32]         ; BootServices->GetMemoryMap
    add rsp, 40

    mov rax, [BootServices]
    mov rcx, [ImageHandle]
    mov rdx, [MapKey]             ; weird ass fucking auth
    sub rsp, 40
	call qword [rax + 232]        ; BootServices->ExitBootServices
    add rsp, 40

    mov rsp, 0x90000              ; move stack far far away from my beautiful amazing awesome kernel 
    
    mov rax, 0x100000             ; read linker.ld 
	jmp rax                       ; mommy rust <3
	
; data for uefi shit
section '.data' data readable writeable

ImageHandle:       dq 0
SystemTable:       dq 0
BootServices:      dq 0

; mem offsets for uefi
MemMapSize:        dq 8192        ; upper bounds
MapKey:            dq 0
DescriptorSize:    dq 0
DescriptorVersion: dd 0
MemMap:            times 8192 db 0

kernel_start:
    file 'kernel.bin'             ; hardcoding :sob: 
kernel_end:
kernel_size = kernel_end - kernel_start
