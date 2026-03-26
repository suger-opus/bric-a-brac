import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";

export const cn = (...inputs: ClassValue[]) => {
  return twMerge(clsx(inputs));
};

// -16 to have some space from the top
export const scrollToElement = (element_id: string, offset: number = -16) => {
  requestAnimationFrame(() => {
    const element = document.getElementById(element_id);
    if (element) {
      const elementPosition = element.getBoundingClientRect().top
        + window.scrollY;
      window.scrollTo({
        top: elementPosition + offset,
        behavior: "smooth"
      });
    }
  });
};


