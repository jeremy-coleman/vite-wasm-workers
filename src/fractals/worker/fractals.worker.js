
import wasm, {
    generate_frame_part,
    FramePartConfig,
    Resolution,
    ComplexPlaneRange,
    GeneratorConfig,
    ColorConfig,
    ColorTransMode
} from 'fractals-wasm'

wasm()


//const wasm = import('fractals-wasm')

onmessage = function (e) {

    wasm().then(() => {

        const startTime = performance.now();

        const conf = FramePartConfig.new(
            Resolution.new(e.data.res[0], e.data.res[1]),
            ComplexPlaneRange.new(
                e.data.plane[0],
                e.data.plane[1],
                e.data.plane[2],
                e.data.plane[3]
            ),
            e.data.scaling,
            e.data.partNum,
            e.data.partCount,
            e.data.maxIters
        )
        const frac = e.data.fractal
        const gen = frac.type === "Mandelbrot"
            ? GeneratorConfig.mandelbrot()
            : GeneratorConfig.julia_set(frac.point[0], frac.point[1])

        const color = ColorConfig.new(
            e.data.color.h,
            e.data.color.s,
            e.data.color.v,
            ColorTransMode[e.data.color.mode],
            e.data.color.smooth
        )

        const partHeight = Math.floor(conf.res.height / conf.part_count);

        let data = generate_frame_part(conf, gen, color);

        postMessage({
            data: data,
            x: 0,
            y:  e.data.partNum * partHeight,
            width: e.data.res[0],
            height: partHeight,
            isFullRes: e.data.isFullRes,
            time: performance.now() - startTime
        });
    }
)}

