"use client";

import { FC } from "react";

import { Profile } from "enities/Profile";

import styles from "./ProfileInfo.module.scss";

interface ProfileInfoProps {
  id: number;
}

export const ProfileInfo: FC<ProfileInfoProps> = ({ id }) => {
  //TODO
  return (
    <div className={styles.wrapper}>
      <Profile id={id} />
    </div>
  );
};
