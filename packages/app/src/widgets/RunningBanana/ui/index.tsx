"use client";
import { FC } from "react";

import { Canvas } from "@react-three/fiber";
import { OrbitControls, Stage } from "@react-three/drei";

import { Banana } from "shared/ui/Banana";

import styles from "./RunningBanana.module.css";

export const RunningBanana: FC = () => {
  return (
    <div className={styles.wrapper}>
      <Canvas>
        <Stage position={[0, 0, 0]}>
          <Banana />
        </Stage>
      </Canvas>
    </div>
  );
};
