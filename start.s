.data
choice:
  .asciz "Choisissez une configuration: (↑ ↓ [ENTRER])\n"
choice_prefix:
  .asciz "  "
intro:
  .asciz "<utilisez les fleches pour vous déplacer>"
clear:
  .asciz "\x1b[H\x1b[2J\x1b[3J\x1b[1;1H"
fin:
  .asciz "\x1b[2K\x1b[1mBravo, vous avez gagné !\x1b[0m"

.global current_maze
current_maze:
  .word 0

.text
.align 2
.global main
.type main,%function
.func main,main
main:
    bl choose_maze_config

    bl show_maze_initial
    ldr r0, =intro
    bl print_string

    ldr r4, =stack
    mov r5, #-10
    str r5, [r4]     // x-1 = -10
    str r5, [r4, #4] // y-1 = -10
    add r4, r4, #8

    // Start pos
    ldr r6, =current_maze
    ldr r6, [r6]
    ldr r7, [r6, #12]
    ldr r6, [r6, #8]
    str r6, [r4]     // x0 = start_x
    str r7, [r4, #4] // y0 = start_y

    // First cell is yellow
    mov r0, r6
    mov r1, r7
    mov r2, #1
    mov r3, #1
    bl draw_cell

    mov r0, r6
    mov r1, r7
    bl move_cursor

    mov r6, #1
    main_loop:
      mov r0, r4
      bl tick
      mov r6, r0
      lsl r0, r0, #3
      add r4, r4, r0

      // Show trail
      cmp r6, #1
      beq ml_p
        ldr r0, [r4, #8]
        ldr r1, [r4, #12]
        mov r2, #1
        mov r3, #0
      b ml_end
      ml_p:
        ldr r0, [r4]
        ldr r1, [r4, #4]
        mov r2, #1
        mov r3, #1
      ml_end:
      bl draw_cell

      // Move cursor
      ldr r0, [r4]
      ldr r1, [r4, #4]
      bl move_cursor

      cmp r6, #0
      bne main_loop

    // Gagné ?
    mov r0, #0
    ldr r1, =current_maze
    ldr r1, [r1]
    ldr r1, [r1, #4]
    add r1, r1, #1
    bl move_cursor
    ldr r0, =fin
    bl print_string
  halt: b halt
.size main,.-main
.endfunc

.type choose_maze_config,%function
.func choose_maze_config,choose_maze_config
choose_maze_config:

.size choose_maze_config,.-choose_maze_config
.endfunc
    push {r4-r11, lr}
    ldr r0, =clear
    bl print_string
    ldr r0, =choice
    bl print_string

    // Iterate over mazes and display their names
    ldr r4, =MAZES
    mov r5, r4
    mov r8, #0 // Number of configs
    cmc_loop:
    ldr r6, [r5]
    cmp r6, #0
    beq cmc_loop_end
      ldr r0, =choice_prefix
      bl print_string
      ldr r0, [r6, #28]
      bl print_string
      mov r0, #'\n'
      bl _writec
      add r5, r5, #4
      add r8, r8, #1
      b cmc_loop
    cmc_loop_end:
    // End iterate

    sub r9, r8, #1 // max choices - 1

    mov r10, #0 // Pseudo-entropy
    mov r5, #0 // Selection

    cmc_a:
      add r10, r10, #1

      mov r0, #2
      add r1, r5, #1
      bl move_cursor
      cmc_invalid_input:
      bl _readc
      cmp r0, #13 // Carriage return
      bne fu
      // Enter pressed
        ldr r0, =lgc_x
        str r10, [r0]
        lsl r5, r5, #2
        add r4, r4, r5
        ldr r4, [r4]
        ldr r6, =current_maze
        str r4, [r6]
        pop {r4-r11, pc}
      // End enter pressed
      fu:
      cmp r0, #27 // ascii escape
      bne cmc_invalid_input
      bl _readc
      cmp r0, #'['
      bne cmc_invalid_input
      bl _readc
      subs r6, r0, #'A'

      bne cmc_a_up
      // up
      cmp r5, #0
      subne r5, r5, #1
      b cmc_a
      // end up
      cmc_a_up:
      cmp r6, #1
      bne cmc_a
      // down
      cmp r5, r9
      addne r5, r5, #1
      // end down
    b cmc_a
.type show_maze_initial,%function
.func show_maze_initial,show_maze_initial
show_maze_initial:
    push {lr}
    ldr r0, =clear
    bl print_string
    bl maze_generate
    ldr r4, =current_maze
    ldr r5, [r4]
    ldr r4, [r5, #0]
    ldr r5, [r5, #4]
    mov r7, #0
    loop_y:
      mov r6, #0
      loop_x:
        mov r0, r6
        mov r1, r7
        mov r2, #0
        bl draw_cell
        add r6, r6, #1
        cmp r6, r4
        blt loop_x
      add r7, r7, #1
      mov r0, #'\n'
      bl _writec
      cmp r7, r5
      blt loop_y
    pop {pc}
.size show_maze_initial,.-show_maze_initial
.endfunc

.type tick,%function
.func tick,tick
tick:
    push {r4-r11, lr}
    mov r4, r0
    tick_invalid_input:
    bl _readc
    cmp r0, #27 // ascii escape
    bne tick_invalid_input
    bl _readc
    cmp r0, #'['
    bne tick_invalid_input
    bl _readc
    subs r5, r0, #'A'

    // position haut de pile
    ldr r8, [r4]     // x_top
    ldr r9, [r4, #4] // y_top

    // switch r5
    bne a_up
    // case "fleche du haut"
    sub r9, r9, #1
    mov r7, #0b0100
    // end case
    b switch_end
    a_up:
    cmp r5, #1
    bne a_down
    // case "fleche du bas"
    add r9, r9, #1
    mov r7, #0b0001
    // end case
    b switch_end
    a_down:
    cmp r5, #2
    bne a_right
    // case "fleche droite"
    add r8, r8, #1
    mov r7, #0b1000
    // end case
    b switch_end
    a_right:
    cmp r5, #3
    bne tick_invalid_input
    // case "fleche gauche"
    sub r8, r8, #1
    mov r7, #0b0010
    // end case
    switch_end:

    // Choses à considérer maintenant
    // 1) la nouvelle position est atteignable par le chemin du labyrinthe ?
    //   - pas besoin de vérifier que la position est dans le terrain
    // 2) la nouvelle position est au même endroit que l'avant dernière ? on pop la pile
    //   - sinon on empile la nouvelle position
    //      - la nouvelle position est l'arrivée ? on retourne 0
    //
    // Puis mettre à jour l'affichage

    // 1: on utilise cell_paths_for_cell qu'on compare avec le masque dans r7
    mov r0, r8
    mov r1, r9
    bl cell_paths_for_cell
    ands r0, r0, r7
    beq tick_invalid_input // si (r7 & cell_paths_for_cell(...)) == 0, le chemin est invalide

    // 2: quoi qu'il en soit, la nouvelle position est écrite dans la pile
    //    on décide après si on incrémente ou qu'on décrémente le pointeur de pile
    str r8, [r4, #8]
    str r9, [r4, #12]

    // fin atteinte ? on retourne 0
    ldr r0, =current_maze
    ldr r0, [r0]
    ldr r1, [r0, #16]
    ldr r0, [r0, #20]
    cmp r8, r1
    cmpeq r9, r0
    moveq r0, #0
    popeq {r4-r11, pc}

    // if
    ldr r10, [r4, #-8]
    cmp r10, r8
    ldr r10, [r4, #-4]
    cmpeq r9, r10
    // then
    moveq r0, #-1 // pop
    popeq {r4-r11, pc}
    // else
    mov r0, #1 // push
    pop {r4-r11, pc}
.size tick,.-tick
.endfunc
