import { type Vector3 } from '@react-three/fiber'

/**
 * Converts spherical coordinates to cartesian
 * @param r Radius value
 * @param theta Theta value in degrees (horizontal plane)
 * @param phi Phi value in degrees (vertical plane)
 * @returns [x, y, z] coordinates
 */
export const sphericalToCartesian = (
  r: number,
  theta: number,
  phi: number
): Vector3 => {
  theta = deg2rad(theta)
  phi = deg2rad(phi)

  const t = r * Math.cos(phi) // distance to y-axis after being rotated up
  const y = r * Math.sin(phi)

  const x = t * Math.cos(theta)
  const z = t * Math.sin(theta)

  return [x, y, z]
}

export const deg2rad = (angle: number): number => angle * (Math.PI / 180)
