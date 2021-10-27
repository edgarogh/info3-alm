#include "maze_configs.h"

#define NULL ((void*) 0L)

static const maze_config MAZE_EASY = {
        .width = 9, .height = 9,
        .start_x = 0, .start_y = 0,
        .end_x = 8, .end_y = 8,
        .custom_mask = NULL,
        .name = "Facile (9x9)",
};

static const maze_config MAZE_HARD = {
        .width = 30, .height = 30,
        .start_x = 0, .start_y = 0,
        .end_x = 29, .end_y = 29,
        .custom_mask = NULL,
        .name = "Difficile (30x30)",
};

#include "polytech.xbm"

static const maze_config MAZE_POLYTECH = {
        .width = xbm_polytech_width, .height = xbm_polytech_height,
        .start_x = xbm_polytech_x_hot, .start_y = xbm_polytech_y_hot,
        .end_x = 5, .end_y = 6,
        .custom_mask = xbm_polytech_bits,
        .name = "Polytech (non-rectangulaire)",
};

const maze_config* MAZES[] = { &MAZE_EASY, &MAZE_HARD, &MAZE_POLYTECH, NULL };
