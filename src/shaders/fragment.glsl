#version 140

const float NIL  = 0.0;
const float OVAL = 1.0;
const float ARCH = 2.0;
const float RECT = 3.0;
const float TRIA = 4.0;

const float TRANS = 5.0;
const float ROTAT = 6.0;
const float ORIGI = 7.0;
const float SCALE = 8.0;
const float COMPL = 9.0;
const float FILL = 10.0;
const float OUTL = 11.0;
const float UNION = 12.0;
const float INTER = 13.0;

in vec2 pos;
uniform sampler1D shapes;
out vec4 color;

vec3 stack[32];
int stack_size = 0;

vec2 translation = vec2(0.0, 0.0);
bool complement = false;
bool intersect = false;

void push(vec3 item) {
    stack[stack_size] = item;
    stack_size++;
}

vec3 pop() {
    stack_size--;
    return stack[stack_size];
}

float peek_instr() {
    return stack[stack_size - 1][0];
}

bool inside_primitive(float shape, float width, float height) {
    if (shape == NIL) {
        return false;

    } else if (shape == RECT) {
        float left = translation.x - width / 2.0;
        float rght = translation.x + width / 2.0;

        float top = translation.y - height / 2.0;
        float bot = translation.y + height / 2.0;

        return pos.x >= left && pos.x <= rght && pos.y >= top && pos.y <= bot;
    }
}

void process_instruction(float instr, float arg1, float arg2) {
    if (instr == TRANS) {
        push(vec3(TRANS, translation));
        translation.x += arg1;
        translation.y += arg2;
    } else if (instr == COMPL) {
        push(vec3(COMPL, 0.0, 0.0));
        complement = !complement;
    }
}

bool pop_instruction() {
    if (stack_size == 0) {
        return false;
    }

    vec3 instruction = pop();
    float instr = instruction[0];
    float arg1 = instruction[1];
    float arg2 = instruction[2];

    if (instr == TRANS) {
        translation = vec2(arg1, arg2);
    } else if (instr == COMPL) {
        complement = !complement;
    }

}

void pop_finished() {
    while (stack_size > 0 && peek_instr() < UNION) { // Only Unions and Intersections apply to more than one shape
        pop_instruction();
    }
}

void main() {
    for (int i = 0; i < textureSize(shapes, 0); i++) {
        vec4 shape = texelFetch(shapes, i, 0);

        float instr = shape[0];
        float arg1  = shape[1];
        float arg2  = shape[2];

        if (instr <= TRIA) {
            if (inside_primitive(instr, arg1, arg2) ^^ complement) {
                color = vec4(1.0, 1.0, 1.0, 1.0);
                return;
            }
            pop_finished();
        } else {
            process_instruction(instr, arg1, arg2);
        }
    }
    color = vec4(0.0, 0.0, 0.0, 1.0);
}