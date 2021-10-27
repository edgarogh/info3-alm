#include "stdbool.h"
#include "stdint.h"
#include "stddef.h"
#ifdef __x86_64__
#include "stdio.h"
#endif

#define maze_width 30
#define maze_height 30

#ifdef mask_width
#undef maze_width
#define maze_width mask_width
#undef maze_height
#define maze_height mask_height
#endif

uint32_t maze_width_ = maze_width;
uint32_t maze_height_ = maze_height;

uint32_t lgc_x = 1; // seed

/**
 * Générateur congruentiel linéaire
 */
uint32_t rand(uint32_t upper_bound) {
    lgc_x *= 22695477;
    lgc_x += 1;
    return lgc_x % upper_bound;
}

/**
 * Type représentant une cellule de labyrithe.
 * Il s'agit d'un champ de bit.
 * Les murs nord et ouest n'ont pas besoin d'être encodés car ils peuvent être trouvés
 * grâce aux cellules adjacentes. Cela enlève de la redondance et limite la marge d'erreur.
 */
typedef struct {
    /** Un mur se trouve à l'est si ce bit est 1 */
    bool wall_east : 1;

    /** Un mur se trouve au sud si ce bit est 1 */
    bool wall_south : 1;
} cell;

typedef union {
    struct {
        bool n : 1, e : 1, s : 1, w : 1;
    };
    char all;
} cell_paths;

cell maze[maze_width * maze_height];
bool maze_visited[maze_width * maze_height] = { false };

cell_paths cell_paths_for_cell(int x, int y) {
    if (x < 0 || x >= maze_width || y < 0 || y >= maze_height) return (cell_paths) {
        .all = 0b0000,
    };

    cell* c = &maze[x + maze_width * y];
    cell* cn = (y > 0) ? &c[-maze_width] : 0;
    cell* cw = (x > 0) ? &c[-1] : 0;

    return (cell_paths) {
        .n = cn ? !cn->wall_south : 0,
        .e = c ? !c->wall_east : 0,
        .s = c ? !c->wall_south : 0,
        .w = cw ? !cw->wall_east : 0,
    };
}

bool is_available(int x, int y) {
    if (x < 0 || x >= maze_width || y < 0 || y >= maze_height) return false;
    return !maze_visited[x + maze_width * y];
}

cell_paths available_paths_for_cell(int x, int y) {
    return (cell_paths) {
        .n = is_available(x, y - 1),
        .e = is_available(x + 1, y),
        .s = is_available(x, y + 1),
        .w = is_available(x - 1, y),
    };
}

typedef struct { uint32_t x, y; } pos;
pos stack[maze_width * maze_height + 1];

void maze_generate() {
    for (int i = 0; i < maze_width * maze_height; i++) {
        maze[i] = (cell) { .wall_east = 1, .wall_south = 1 };
    }

    // Initialisation de la pile
    uint32_t start_x =
    #ifdef mask_x_hot
    mask_x_hot; // xbm hotspot x
    #else
    rand(maze_width);
    #endif

    uint32_t start_y =
    #ifdef mask_y_hot
    mask_y_hot; // xbm hotspot y
    #else
    rand(maze_height);
    #endif

    stack[0] = (pos) { start_x, start_y };

    // Boucle de génération utilisant la pile
    for (int stack_index = 0; stack_index >= 0;) {
        char x = stack[stack_index].x, y = stack[stack_index].y;
        maze_visited[x + maze_width * y] = true;
        int idx = x + maze_width * y;
        cell_paths paths = available_paths_for_cell(stack[stack_index].x, stack[stack_index].y);
        if ((paths.all & 0b1111) == 0) {
            stack_index--;
        } else {
            char n = paths.n, e = paths.e, s = paths.s, w = paths.w;
            char wall_index = rand(n + e + s + w);
            pos new_pos = { .x = x, .y = y };

            if (n && wall_index-- == 0) {
                maze[idx - maze_width].wall_south = false;
                new_pos.y--;
            } else if (e && wall_index-- == 0) {
                maze[idx].wall_east = false;
                new_pos.x++;
            } else if (s && wall_index-- == 0) {
                maze[idx].wall_south = false;
                new_pos.y++;
            } else {
                maze[idx - 1].wall_east = false;
                new_pos.x--;
            }

            stack[++stack_index] = new_pos;
        }
    }
}

// Caractères UTF8 de 3 octets chacuns représentant les murs du labyrinthe
// Ils sont ordonés tel qu'une texture peut être recupérée en indexant l'élément
// dont l'indice a pour bits {n,e,s,w}, où chaque point cardinal représente
// l'ABSENCE d'un mur.
// Par exemple, si cell_texture = {0, 1, 1, 1}, l'avant dernier caractère sera
// recupéré, soit celui pointant vers ouest/sud/est.
// Chaque "texture" fait 3 octets de long car il s'agit de caractères
// utf8 du bloc unicode "box drawings", à l'exception du premier qui est représenté
// par une espace (mais qui ne devrait jamais arriver)
static const char CELL_TEXTURES[3 * 16] = " \0\0║═╚║║╔╠═╝═╩╗╣╦╬";

#ifdef __arm__
void _writec(char c);
void print_string(char*);

void write_number(int n) {
    if (n > 999) {
        _writec('%');
        while (1);
    }

    char c0 = (n / 100);
    char c1 = ((n - (100 * c0)) / 10);
    char c2 = (n - (10 * c1));
    if (c0) _writec(c0 + '0');
    if (c1 || c0) _writec(c1 + '0');
    if (c2 || c1 || c0) _writec(c2 + '0');
}

void move_cursor(int x, int y) {
    _writec(27);
    _writec('[');
    write_number(y + 1);
    _writec(';');
    write_number(x + 1);
    _writec('H');
}
#endif

void draw_cell(int x, int y, bool position, bool emphasis) {
    size_t tex = cell_paths_for_cell(x, y).all & 0b1111;
    tex *= 3;

#ifdef __x86_64__
    printf("%c%c%c", CELL_TEXTURES[tex], CELL_TEXTURES[tex+1], CELL_TEXTURES[tex+2]);
#elif defined __arm__
    if (x == 29 && y == 29) {
        print_string("▶");
        return;
    }

    if (position) {
        move_cursor(x, y);

        _writec(27);
        _writec('[');
        if (emphasis) {
            _writec('3');
            _writec('3');
        } else {
            _writec('0');
        }
        _writec('m');
    }
    
    _writec(CELL_TEXTURES[tex+0]);
    _writec(CELL_TEXTURES[tex+1]);
    _writec(CELL_TEXTURES[tex+2]);
#endif
}

#ifdef __x86_64__
void show_maze() {
    for (int y = 0; y < maze_height; y++) {
        for (int x = 0; x < maze_width; x++) {
            draw_cell(x, y, 0);
        }
        printf("\n");
    }
}

int main() {
    #ifdef mask_width
    // Bits par ligne xbm
    const size_t line_bytes = (maze_width + 8 - 1) / 8;
    for (size_t y = 0; y < maze_height; y++) {
        for (size_t x = 0; x < maze_width; x++) {
            size_t ai = line_bytes * y + x / 8;
            size_t ab = 1 << (x % 8);
            bool v = (mask_bits[ai] & ab) == 0;
            maze_visited[x + y * maze_width] = v;
        }
    }
    #endif

    maze_generate();
    show_maze();

    return 0;
}
#endif
