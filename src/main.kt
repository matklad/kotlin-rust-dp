import java.io.File
import kotlin.system.measureTimeMillis

fun squareDist(xs: DoubleArray, xidx: Int, ys: DoubleArray, yidx: Int): Double {
    val dif = xs[xidx] - ys[yidx]
    return dif * dif
}

fun min3(x: Double, y: Double, z: Double): Double =
        if (x < y) {
            if (x < z) x else z
        } else {
            if (y < z) y else z
        }

fun computeDtw(xs: DoubleArray, ys: DoubleArray): Double {
    check(xs.size == ys.size)
    val n = xs.size
    var curr = DoubleArray(n)
    var prev = DoubleArray(n)

    curr[0] = squareDist(xs, 0, ys, 0)
    for (i in 1 until n) {
        curr[i] = curr[i - 1] + squareDist(xs, 0, ys, i)
    }

    // --- Compute DTW
    for (idx_line in 1 until n) {
        val tmp = curr
        curr = prev
        prev = tmp
        curr[0] = prev[0] + squareDist(xs, idx_line, ys, 0)
        for (idx_col in 1 until n) {
            val d11 = prev[idx_col - 1]
            val d01 = curr[idx_col - 1]
            val d10 = prev[idx_col]
            curr[idx_col] = min3(d11, d01, d10) + squareDist(xs, idx_line, ys, idx_col)
        }
    }
    return curr[n - 1]
}


fun main(args: Array<String>) {
    val vec: List<DoubleArray> = File("50.csv").useLines { lines ->
        lines.drop(1).map {
            it.split(',').drop(1).map(String::toDouble).toList().toDoubleArray()
        }.toList()
    }
    // --- 2: Compute sum of DTW
    var totalE: Double = 0.0

    val elapsed = measureTimeMillis {
        for ((id, vi) in vec.withIndex()) {
            for (vj in vec.drop(id)) {
                totalE += computeDtw(vi, vj)
            }
        }
    }
    println("$elapsed ms")
    println("Total error: $totalE")
}
