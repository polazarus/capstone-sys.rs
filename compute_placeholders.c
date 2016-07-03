#include <stdio.h>
#include <stddef.h>
#include <stdint.h>

#include <capstone.h>

int compute_align(const char* name, size_t start, size_t end) {
  int align = 1;
  size_t size = end - start;
  while ((start & align) == 0 && (size & align) == 0) {
    align = align << 1;
  }
  if (align > 8) align = 8;
  printf("pub type %s = [u%d; %d];\n", name, align<<3, size/align);

}

int main() {
  compute_align("detail_data", offsetof(cs_detail, x86), sizeof(cs_detail));

  compute_align("arm64_op_data", offsetof(cs_arm64_op, reg), sizeof(cs_arm64_op));
  compute_align("arm_op_data", offsetof(cs_arm_op, reg), offsetof(cs_arm_op, subtracted));
  compute_align("mips_op_data", offsetof(cs_mips_op, reg), sizeof(cs_mips_op));
  compute_align("ppc_op_data", offsetof(cs_ppc_op, reg), sizeof(cs_ppc_op));
  compute_align("sparc_op_data", offsetof(cs_sparc_op, reg), sizeof(cs_sparc_op));
  compute_align("sysz_op_data", offsetof(cs_sysz_op, reg), sizeof(cs_sysz_op));
  compute_align("x86_op_data", offsetof(cs_x86_op, reg), offsetof(cs_x86_op, size));
  compute_align("xcore_op_data", offsetof(cs_xcore_op, reg), sizeof(cs_xcore_op));
}
