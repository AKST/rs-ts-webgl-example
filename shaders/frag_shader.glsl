precision mediump float;

varying vec4 v_color;

void main() {
    gl_FragColor = v_color + vec4(0.25, 0.25, 0.25, 0.25);
}
