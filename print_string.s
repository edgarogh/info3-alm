.text
.global print_string
.type print_string,%function
.func print_string,print_string

.global print_string
print_string:
push {fp, lr}
mov r2, r0

boucle:
ldrb r0, [r2]
cmp r0, #0
popeq {fp, lr}
bxeq lr
bl _writec

add r2, r2, #1
b boucle

.size print_string,.-print_string
.endfunc

.string_ptr:
    .word string
string:
    .ascii "CA MARCHE!\n\000"
