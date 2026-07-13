format pe64 dll efi
entry main

section '.text' code readable executable

main:
    mov [ImageHandle], rcx
    mov [SystemTable], rdx

    mov rax, [rdx + 96]           ; SystemTable->BootServices
    mov [BootServices], rax
	
	mov rax, [BootServices]
	lea rcx, [GOP_GUID]
	xor rdx, rdx
	lea r8, [GopInterface]

	sub rsp, 40
	call qword [rax + 320]
	add rsp, 40 ; locateProtocol
	test rax, rax
	jnz .error
	
    mov rax, [GopInterface]
    mov rbx, [rax + 24]            ; rbx = GopInterface->Mode pointer

    mov rcx, [rbx + 24]            ; rcx = Mode->FrameBufferBase
    mov [fb_base], rcx

    mov rdx, [rbx + 8]             ; rdx = Mode->Info pointer

    mov ecx, [rdx + 4]             ; Info->HorizontalResolution
    mov [fb_width], ecx

    mov ecx, [rdx + 8]             ; Info->VerticalResolution
    mov [fb_height], ecx

    mov ecx, [rdx + 12]            ; Info->PixelFormat
    mov [fb_format], ecx

    mov ecx, [rdx + 32]            ; Info->PixelsPerScanLine
    mov [fb_stride], ecx

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
	
	cli
	cld
    mov rsp, 0x90000              ; move stack far far away from my beautiful amazing awesome kernel 
   	
	sub rsp, 8
	lea rdi, [BootInfoStruct]      ; pass boot info struct to kernel
    mov rax, 0x100000             ; read linker.ld 
	jmp rax                       ; mommy rust <3

.error:
	xor rax, rax
	mov  [rax], rax

	cli
	hlt
	jmp .error

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

GOP_GUID:
	dd 0x9042A9DE          ; (4 bytes)
	dw 0x23DC              ; (2 bytes)
	dw 0x4A38              ; (2 bytes)
	db 0x96, 0xFB, 0x7A, 0xDE, 0xD0, 0x80, 0x51, 0x6A ; (8 bytes)

GopInterface dq 0          ; GOP ptr

align 8
BootInfoStruct:
fb_base:           dq 0          ; (8 bytes)
fb_width:          dd 0          ; (4 bytes)
fb_height:         dd 0          ; (4 bytes)
fb_stride:         dd 0          ; (4 bytes)
fb_format:         dd 0          ; (4 bytes)


align 16
kernel_start:
    file 'kernel.bin'             ; hardcoding :sob: 
kernel_end:
kernel_size = kernel_end - kernel_start
