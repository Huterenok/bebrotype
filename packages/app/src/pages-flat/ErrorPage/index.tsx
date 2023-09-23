"use client";

import { FC, useEffect } from "react";
import Image from "next/image";

import { Button } from "shared/ui";
import styles from "./ErrorPage.module.scss";

interface ErrorPageProps {
  error: Error;
  reset: () => void;
}

export const ErrorPage: FC<ErrorPageProps> = ({ error, reset }) => {
  useEffect(() => {
    console.log(error);
  }, [error]);

  return (
    <div className={styles.wrapper}>
      <Image width={64} height={64} alt="Sad :(" src="/icons/sad-icon.svg" />
      <h2>Something went wrong!</h2>
      <Button onClick={reset}>Try again</Button>
    </div>
  );
};
