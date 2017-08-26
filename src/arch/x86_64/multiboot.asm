  section .multiboot_header
header_start:
  dd 0xe85250d6                 ; magic number (multiboot 2)
  dd 0                          ; architecture 0 (protected mode i386)
  dd header_end - header_start  ; header length

  ;;  checksum
  ;;  we're using a hack here to keep the compiler happy
  dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))

  ;; optional multiboot 2 tags go here

  ;; end tag
  dw 0                          ; type
  dw 0                          ; flags
  dd 8                          ; size
header_end:
