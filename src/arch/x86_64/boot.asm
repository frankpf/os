  global start

  section .text
  bits 32
start:
  ;; Print 'Yo' to the screen
  mov dword [0xb8000], 0x2f6f2f59
  hlt
