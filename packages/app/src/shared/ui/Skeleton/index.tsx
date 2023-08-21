import { CSSProperties, FC } from "react";

import cls from "./Skeleton.module.scss";
import classNames from "classnames";

interface SkeletonProps {
  className?: string;
  height?: string | number;
  width?: string | number;
  border?: string;
}

export const Skeleton: FC<SkeletonProps> = ({ className, height, width, border }) => {
  const styles: CSSProperties = {
    width,
    height,
    borderRadius: border,
  };

  return (
    <div className={classNames(cls.Skeleton, {}, [className])} style={styles} />
  );
};
