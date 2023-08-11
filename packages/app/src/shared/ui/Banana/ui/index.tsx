"use client";

import React, { FC, useEffect, useRef } from "react";

import { useGLTF, useAnimations, Stage } from "@react-three/drei";
import { Canvas, useFrame } from "@react-three/fiber";
import { Group } from "three";

export const Banana: FC = (props) => {
  const group = useRef<Group>(null);
  //@ts-ignore
  const { nodes, materials, animations } = useGLTF(
    "/models/banana/scene-transformed.glb"
  );
  const { actions } = useAnimations(animations, group);

  useEffect(() => {
    actions["metarigAction.001"]?.play();
  }, []);

  useFrame(() => {
    group.current?.rotateY(0.01);
  });

  return (
    //TODO: Fix bug
    <group ref={group} {...props} dispose={null} position={[-0.5, -3.5, 0.65]}>
      <group name="Sketchfab_Scene">
        <primitive object={nodes.GLTF_created_0_rootJoint} />
        <skinnedMesh
          name="Object_7"
          geometry={nodes.Object_7.geometry}
          material={materials["Material.002"]}
          skeleton={nodes.Object_7.skeleton}
          position={[0.228, 3.384, 0]}
        />
        <skinnedMesh
          name="Object_8"
          geometry={nodes.Object_8.geometry}
          material={materials["Material.006"]}
          skeleton={nodes.Object_8.skeleton}
          position={[0.228, 3.384, 0]}
        />
        <skinnedMesh
          name="Object_10"
          geometry={nodes.Object_10.geometry}
          material={materials["Material.003"]}
          skeleton={nodes.Object_10.skeleton}
          position={[0.228, 3.384, 0]}
        />
        <skinnedMesh
          name="Object_11"
          geometry={nodes.Object_11.geometry}
          material={materials["Material.006"]}
          skeleton={nodes.Object_11.skeleton}
          position={[0.228, 3.384, 0]}
        />
        <skinnedMesh
          name="Object_13"
          geometry={nodes.Object_13.geometry}
          material={materials["Material.005"]}
          skeleton={nodes.Object_13.skeleton}
          position={[0.228, 3.384, 0]}
        />
        <skinnedMesh
          name="Object_14"
          geometry={nodes.Object_14.geometry}
          material={materials["Material.006"]}
          skeleton={nodes.Object_14.skeleton}
          position={[0.228, 3.384, 0]}
        />
        <skinnedMesh
          name="Object_16"
          geometry={nodes.Object_16.geometry}
          material={materials["Material.004"]}
          skeleton={nodes.Object_16.skeleton}
          position={[0.228, 3.384, 0]}
        />
        <skinnedMesh
          name="Object_17"
          geometry={nodes.Object_17.geometry}
          material={materials["Material.006"]}
          skeleton={nodes.Object_17.skeleton}
          position={[0.228, 3.384, 0]}
        />
        <skinnedMesh
          name="Object_19"
          geometry={nodes.Object_19.geometry}
          material={materials["Material.001"]}
          skeleton={nodes.Object_19.skeleton}
          position={[0.228, 3.384, 0]}
        />
        <skinnedMesh
          name="Object_20"
          geometry={nodes.Object_20.geometry}
          material={materials["Material.007"]}
          skeleton={nodes.Object_20.skeleton}
          position={[0.228, 3.384, 0]}
        />
      </group>
    </group>
  );
};

useGLTF.preload("/scene-transformed.glb");
