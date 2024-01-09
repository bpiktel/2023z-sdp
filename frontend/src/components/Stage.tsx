import { useMemo, useRef, useState } from "react";
import { Canvas, type Vector3, useLoader } from "@react-three/fiber";
import { Box, OrbitControls, Torus } from "@react-three/drei";
import { type Mesh } from "three";
import { OBJLoader } from "three/examples/jsm/Addons.js";
import { deg2rad, sphericalToCartesian } from "../utils/mathUtils";

const MeshHATS = ({ position }: { position: Vector3 }): JSX.Element => {
  const mesh = useRef<Mesh>(null);
  const obj = useLoader(OBJLoader, "/meshes/hats.obj");

  return (
    <mesh ref={mesh} position={position}>
      <primitive object={obj} />
    </mesh>
  );
};

const TargetSphere = ({
  position,
  azimuth,
  elevation
}: {
  position: Vector3;
  azimuth: number;
  elevation: number;
}): JSX.Element => {
  const meshRef = useRef<any>();

  const [hovered, setHover] = useState(false);
  const [active, setActive] = useState(false);

  return (
    <mesh
      position={position}
      ref={meshRef}
      scale={active ? 0.4 : 0.25}
      onClick={(event) => {
        setActive(!active);
        console.log("azimuth", azimuth, "elevation", elevation);
      }}
      onPointerOver={(event) => {
        setHover(true);
      }}
      onPointerOut={(event) => {
        setHover(false);
      }}
    >
      <sphereGeometry args={[1]} />
      <meshStandardMaterial color={active || hovered ? "yellow" : "white"} />
    </mesh>
  );
};

export const Stage = (): JSX.Element => {
  const DIVISIONS_AZIMUTH = 12;
  const DIVISIONS_ELEVATION = 8;
  const RADIUS = 10;
  const azimuthAngles = Array.from(
    { length: DIVISIONS_AZIMUTH },
    (_, i) => i * (360 / DIVISIONS_AZIMUTH) - 180
  );
  const elevationAngles = Array.from(
    { length: DIVISIONS_ELEVATION + 1 },
    (_, i) => (i * 180) / DIVISIONS_ELEVATION - 90
  );

  return (
    <div className="flex w-full h-full bg-black">
      <Canvas camera={{ position: [-2, 10, -15] }}>
        <OrbitControls />
        <ambientLight intensity={1} />
        <directionalLight position={[0, 15, 0]} />
        <pointLight position={[0, 5, 0]} intensity={1} />

        <MeshHATS position={[0, -1.5, 0]} />
        <Box position={[0, -2.5, 0]} scale={[4, 0.5, 4]}>
          <meshStandardMaterial color={"orange"} />
        </Box>

        {/* {azimuthAngles.map((theta) =>
          elevationAngles.map((phi) => {
            if ((phi === -90 || phi === 90) && theta !== 0) return null; // remove duplicate points on top and bottom
            return (
              <TargetSphere
                key={`theta:${theta}-phi:${phi}`}
                position={sphericalToCartesian(RADIUS, theta + 90, phi)} // rotate theta 90 degrees to match viewing angle
                azimuth={theta}
                elevation={phi}
              />
            );
          })
        )} */}

        <Torus
          position={[0, 0, 0]}
          args={[RADIUS, 0.03]}
          rotation={[Math.PI / 2, 0, 0]}
        >
          <meshStandardMaterial color={"#00d2ff"} />
        </Torus>
        {/* {azimuthAngles.map((theta) => (
          <Torus
            key={`ring${theta}`}
            position={[0, 0, 0]}
            args={[RADIUS, 0.03]}
            rotation={[0, deg2rad(theta), 0]}
          >
            <meshStandardMaterial color={"#9bedff"} />
          </Torus>
        ))} */}
      </Canvas>
    </div>
  );
};
