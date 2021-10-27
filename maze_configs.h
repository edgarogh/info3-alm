#include "stdint.h"

typedef struct {
    uint32_t width, height;
    uint32_t start_x, start_y;
    uint32_t end_x, end_y;
    unsigned char* custom_mask;
    char* name;
} maze_config;

extern const maze_config* MAZES[];
extern const maze_config* current_maze;
