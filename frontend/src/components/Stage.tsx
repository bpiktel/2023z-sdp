import { Box, OrbitControls, Torus, Text } from "@react-three/drei";
import {
  Canvas,
  useFrame,
  useLoader,
  type Vector3,
  useThree
} from "@react-three/fiber";
import { useEffect, useRef, useState } from "react";
import * as THREE from "three";
import { Euler, type Mesh } from "three";
import { OBJLoader } from "three/examples/jsm/Addons.js";
import { deg2rad, sphericalToCartesian } from "../utils/mathUtils";
import { SphericalCoordinates } from "schemas/coordinates";

const MeshHATS = ({
  position,
  rotation
}: {
  position: Vector3;
  rotation?: [number, number, number];
}): JSX.Element => {
  const mesh = useRef<Mesh>(null);
  const obj = useLoader(OBJLoader, "/meshes/hats.obj");

  return (
    <mesh
      ref={mesh}
      position={position}
      rotation={rotation && new Euler(rotation[0], rotation[1], rotation[2])}
    >
      <primitive object={obj} />
    </mesh>
  );
};

const TargetSphere = ({
  position,
  onClick,
  active,
  highlight,
  isHoverDisabled
}: {
  position: Vector3;
  onClick: () => void;
  active?: boolean;
  highlight?: boolean;
  isHoverDisabled?: boolean;
}): JSX.Element => {
  const [hovered, setHover] = useState(false);

  const getColor = (): string => {
    if (hovered) return "yellow";
    if (highlight) return "red";
    if (active) return "yellow";
    return "white";
  };

  return (
    <mesh
      position={position}
      scale={hovered ? 0.4 : 0.25}
      onClick={() => {
        onClick();
      }}
      onPointerOver={() => {
        !isHoverDisabled && setHover(true);
      }}
      onPointerOut={() => {
        setHover(false);
      }}
    >
      <sphereGeometry args={[1]} />
      <meshStandardMaterial color={getColor()} />
    </mesh>
  );
};

const StageContent = ({
  selection,
  setSelection,
  highlight,
  currentSample
}: {
  selection: SphericalCoordinates | null;
  setSelection: (selection: SphericalCoordinates) => void;
  highlight: SphericalCoordinates | null;
  currentSample: "start" | "end" | number;
}): JSX.Element => {
  const DIVISIONS_AZIMUTH = 24;
  const DIVISIONS_ELEVATION = 12;
  const RADIUS = 10;
  const azimuthAngles = Array.from(
    { length: DIVISIONS_AZIMUTH },
    (_, i) => i * (360 / DIVISIONS_AZIMUTH)
  );
  const elevationAngles = Array.from(
    { length: DIVISIONS_ELEVATION + 1 },
    (_, i) => (i * 180) / DIVISIONS_ELEVATION - 90
  );

  const three = useThree();
  const [clock] = useState(new THREE.Clock());
  const FPS_CAP = 30;

  useEffect(() => {
    three.camera.position.set(-2, 10, -15);
  }, [currentSample]);

  useFrame(({ gl, scene, camera }) => {
    const timeUntilNextFrame = 1000 / FPS_CAP - clock.getDelta();

    setTimeout(() => {
      gl.render(scene, camera);
    }, Math.max(0, timeUntilNextFrame));
  }, 1);
  return (
    <>
      <OrbitControls enablePan={false} />
      <ambientLight intensity={1} />
      <directionalLight position={[0, 15, 0]} />
      <pointLight position={[0, 5, 0]} intensity={1} />

      <MeshHATS position={[0, -1.5, 0]} />
      <Box position={[0, -2.5, 0]} scale={[4, 0.5, 4]}>
        <meshStandardMaterial color={"orange"} />
      </Box>

      {azimuthAngles.map((theta) =>
        elevationAngles.map((phi) => {
          if ((phi === -90 || phi === 90) && theta !== 0) return null; // remove duplicate points on top and bottom
          return (
            <TargetSphere
              key={`theta:${theta}-phi:${phi}`}
              position={sphericalToCartesian(RADIUS, theta + 90, phi)} // rotate theta 90 degrees to match viewing angle
              onClick={() => {
                !highlight && setSelection({ azimuth: theta, elevation: phi });
              }}
              active={
                selection?.azimuth === theta && selection?.elevation === phi
              }
              highlight={
                highlight?.azimuth === theta && highlight?.elevation === phi
              }
              isHoverDisabled={!!highlight}
            />
          );
        })
      )}
      <Torus
        position={[0, 0, 0]}
        args={[RADIUS, 0.03]}
        rotation={[Math.PI / 2, 0, 0]}
      >
        <meshStandardMaterial color={"#00d2ff"} />
      </Torus>
      {azimuthAngles.map((theta) => (
        <Torus
          key={`ring${theta}`}
          position={[0, 0, 0]}
          args={[RADIUS, 0.03]}
          rotation={[0, deg2rad(theta + 90), 0]}
        >
          <meshStandardMaterial color={"#9bedff"} />
        </Torus>
      ))}
      {azimuthAngles.map(
        (theta) =>
          theta % 30 === 0 && (
            <Text
              key={`text${theta}`}
              position={[
                -Math.sin(deg2rad(theta + 1)) * RADIUS,
                0.5,
                Math.cos(deg2rad(theta + 1)) * RADIUS
              ]}
              rotation={[0, -Math.PI / 2 - deg2rad(theta + 90), 0]}
              anchorX="left"
              fontSize={0.75}
            >
              {theta}°
            </Text>
          )
      )}
    </>
  );
};

export const Stage = ({
  selection,
  setSelection,
  highlight,
  currentSample
}: {
  selection: SphericalCoordinates | null;
  setSelection: (selection: SphericalCoordinates) => void;
  highlight: SphericalCoordinates | null;
  currentSample: "start" | "end" | number;
}): JSX.Element => {
  return (
    <div className="flex w-full h-full bg-black">
      <Canvas camera={{ position: [-2, 10, -15] }} frameloop="demand">
        <StageContent
          selection={selection}
          setSelection={setSelection}
          highlight={highlight}
          currentSample={currentSample}
        />
      </Canvas>
    </div>
  );
};
