---
title: Bidirectional reflectance distribution function
date: 2026-07-30
updated: 2026-07-30
tags:
  - graphics
  - math
  - physics
  - statistics
  - teaching
---
$$
\begin{equation}
\textcolor{#fe7fb3}{f_r(\omega_i,\omega_r)}
=
\frac{
d\textcolor{#0466e7}{L_r(\omega_r)}
}{
d\textcolor{#ffbc3f}{E_i(\omega_i)}
}
=
\frac{
d\textcolor{#0466e7}{L_r(\omega_r)}
}{
\textcolor{#dd6fff}{L_i(\omega_i)}
\textcolor{#9dc141}{\cos\theta_i}
\textcolor{#7be9ff}{\,d\omega_i}
}
\end{equation}
$$

To find <span style="color:#fe7fb3">how much light is reflected by a surface</span>, we divide <span style="color:#0466e7">the outgoing radiance</span> by <span style="color:#ffbc3f">the incoming irradiance</span>. Since irradiance is itself equal to <span style="color:#dd6fff">the incoming radiance</span> multiplied by <span style="color:#9dc141">the cosine of the incident angle</span> and <span style="color:#7be9ff">the differential solid angle</span>, the second form is often more useful in rendering. 
### Legend
- $\textcolor{#fe7fb3}{f_r(\omega_i,\omega_r)}$ - Bidirectional Reflectance Distribution Function. 
- $\textcolor{#0466e7}{L_r(\omega_r)}$ - Reflected (outgoing) radiance. 
- $\textcolor{#ffbc3f}{E_i(\omega_i)}$ - Incoming irradiance. 
- $\textcolor{#dd6fff}{L_i(\omega_i)}$ - Incoming radiance. 
- $\textcolor{#9dc141}{\cos(\theta_i)}$ - Cosine term. 
- $\textcolor{#7be9ff}{d\omega_i}$ - Differential solid angle.

A perfectly diffuse surface has the same BRDF regardless of direction, while glossy or metallic materials produce much larger values around their preferred reflection directions. Different BRDFs therefore produce different material appearances while still fitting into the same rendering equation. For a given incoming ray, the surface may absorb some energy, reflect the ray in some direction or transmit it through the material (transmission is not handled by the BRDF). The rendering equation multiplies the incoming radiance by the BRDF because not every incoming photon contributes equally to what the viewer ultimately sees. 

A BRDF cannot reflect more light than it receives,
$$
{\textcolor{#7be9ff}{\int_\Omega}}
\textcolor{#fe7fb3}{f_r}(
\textcolor{#7be9ff}{\omega_i}, \textcolor{#0466e7}{\omega_o})
\textcolor{#9dc141}{\cos(\theta)}
\,\textcolor{#7be9ff}{d\omega_i}
\le 1
$$
Swapping the incoming and outgoing directions should not change the BRDF.

$$
\textcolor{#fe7fb3}{f_r}(
\textcolor{#7be9ff}{\omega_i},
\textcolor{#0466e7}{\omega_o}
)
=
\textcolor{#fe7fb3}{f_r}(
\textcolor{#0466e7}{\omega_o},
\textcolor{#7be9ff}{\omega_i}
)
$$


The simplest possible BRDF is the Lambertian model.

$$
\textcolor{#fe7fb3}{f_r}
=
\frac{\textcolor{#f39c12}{\rho}}{\pi}
$$
where

- $\textcolor{#f39c12}{\rho}$ is the surface albedo

