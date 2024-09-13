@main
def main(): Unit = {

  val startTime = System.currentTimeMillis()

  val filename = "10-point-pairs.json"
  val bufferedSource = scala.io.Source.fromFile(filename)

  val it = bufferedSource.iterator
  val sum = parseInput(it)
  println(s"Sum of distances: $sum")

  val totalTime = System.currentTimeMillis() - startTime
  println(s"Total time: ${totalTime / 1000}s")
  bufferedSource.close()
}

def getNextNumber(it: Iterator[Char]): Double = {
  forwardToColon(it)
  parseNumber(it)
}

def forwardToColon(it: Iterator[Char]): Unit = {
  while (it.hasNext && it.next() != ':') {}
}

def parseNumber(it: Iterator[Char]): Double = {
  val sb = new StringBuilder
  while (it.hasNext) {
    val c = it.next()
    if (c == ',' || c == '}' || c == ']') {
      return sb.toString.toDouble
    }
    sb += c
  }
  sb.toString.toDouble
}

def parseInput(it: Iterator[Char]): Double = {
  while (it.hasNext && it.next() != '[') {} // fast forward to `[`

  var total = 0.0
  while (it.hasNext) {
    if (it.next() == ']') {
      return total// we're done with the array
    }
    val x0 = getNextNumber(it)
    val y0 = getNextNumber(it)
    val x1 = getNextNumber(it)
    val y1 = getNextNumber(it)
    val d = getNextNumber(it)

    val dist = haversineDistance((x0, y0), (x1, y1))
    if (dist != d) {
      println(s"Distance mismatch: $dist != $d")
    }
    total += dist
  }
  -1.0 // something went wrong
}

def haversineDistance(p1: (Double, Double), p2: (Double, Double)): Double = {
  val lat1 = p1._1
  val lon1 = p1._2
  val lat2 = p2._1
  val lon2 = p2._2
  val R = 6371 // Radius of the earth in km
  val dLat = (lat2 - lat1).toRadians
  val dLon = (lon2 - lon1).toRadians
  val a = Math.sin(dLat / 2) * Math.sin(dLat / 2) +
    Math.cos(lat1.toRadians) * Math.cos(lat2.toRadians) *
      Math.sin(dLon / 2) * Math.sin(dLon / 2)
  val c = 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a))
  val distance = R * c // Distance in km
  distance
}

