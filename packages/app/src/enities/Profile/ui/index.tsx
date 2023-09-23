"use client";

import { FC, useEffect } from "react";
import Image from "next/image";
import { useUnit } from "effector-react";

import {
  $error,
  $isPending,
  $profile,
  getProfileEv,
  setProfileEv,
} from "../model";
import { IProfile } from "../types";

import { Bar } from "shared/ui";
import styles from "./Profile.module.scss";

interface ProfileProps {
  id: number;
  currentUser: IProfile | null;
}

//TODO: change name
export const Profile: FC<ProfileProps> = ({ id, currentUser }) => {
  const [profile, error, isPending, setProfile, getProfile] = useUnit([
    $profile,
    $error,
    $isPending,
    setProfileEv,
    getProfileEv,
  ]);

  useEffect(() => {
    if (currentUser) {
      setProfile(currentUser);
    } else {
      getProfile(id);
    }
  }, [id, currentUser]);

  //TODO
  if (error) {
    return <div>Something went wrong</div>;
  }

  //TODO
  return isPending ? (
    <div>Loading...</div>
  ) : (
    <div className={styles.wrapper}>
      <Image
        width={256}
        height={256}
        src={profile?.avatar ?? "/icons/profile.svg"}
        alt="Profile"
        className={styles.image}
      />
      <div className={styles.fields}>
        {/* TODO: should username and email be changable? */}
        <p className={styles.field}>
          Username: <Bar>{profile?.username}</Bar>
        </p>
        {/* Only current user can see */}
        {currentUser && (
          <p className={styles.field}>
            Email: <Bar>{profile?.email}</Bar>
          </p>
        )}
        {/* TODO */}
        {profile?.near_address ?? "Don't have any registered NEAR address"}
      </div>
    </div>
  );
};
