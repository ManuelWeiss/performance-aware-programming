@main
def main(): Unit = {
  val startTime = System.currentTimeMillis()
  val numberOfPoints = 10e6.toInt
  val numberOfCluster = 100
  val pointsPerCluster = numberOfPoints / numberOfCluster
  val diameterOfCluster = 10

  val file = java.io.FileWriter("10M-point-pairs.json")
  file.write("""{"pairs":[""")

  var sum = 0.0
  for (i <- 1 to numberOfCluster) {
    val center1 = generateRandomPoint()
    val center2 = generateRandomPoint()
    println(s"Cluster centers $i: $center1, $center2")
    for (j <- 1 to pointsPerCluster) {
      val p1 = generateRandomPointAround(center1, diameterOfCluster)
      val p2 = generateRandomPointAround(center2, diameterOfCluster)
      val dist = haversineDistance(p1, p2)
      sum += dist
      file.write(s"""{"x0":${p1._1},"y0":${p1._2},"x1":${p2._1},"y1":${p2._2},"d":$dist}""")
      if (i != numberOfCluster || j != pointsPerCluster) {
        file.write(",\n")
      }
    }
  }

  file.write("],\n")
  file.write(s""""sum":$sum}""")
  file.close()
  val totalTime = System.currentTimeMillis() - startTime
  println(s"Total time: ${totalTime / 1000}s")
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

def generateRandomPoint(): (Double, Double) = {
  val lat = scala.util.Random.nextDouble() * 180 - 90
  val long = scala.util.Random.nextDouble() * 360 - 180
  (lat, long)
}

def generateRandomPointAround(center: (Double, Double), size: Int): (Double, Double) = {
  val lat = center._1 + scala.util.Random.nextDouble() * size - size / 2
  val long = center._2 + scala.util.Random.nextDouble() * size - size / 2
  (lat, long)
}
