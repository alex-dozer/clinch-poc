/*

mint file for minting a pipeline component. can be any name but necessary for the main to have the file name.

*/

use common::data_objs::{Artifact, LuciusContext};
use lucius_macro::lunch;

lunch! {

    component = lstran

    {

    meta {
        name    = "structural_probe_poc"
        author  = "dozer-project"
        version = "0.2.0"
        scope   = any
    }

    operations {

        operation magic {
            do inspect_magic   output magic_probe
            do classify_format output format_probe
            do entropy_probe   output entropy_probe
        }

    }

    signals {

        family format {

            signal pdf_magic {
                derive from operation.magic.inspect_magic
                    when magic_probe.magic == [0x25, 0x50, 0x44, 0x46]
            }

            signal pe_magic {
                derive from operation.magic.inspect_magic
                    when magic_probe.magic[0] == 0x4D
                      && magic_probe.magic[1] == 0x5A
            }

            signal classified_pdf {
                derive from operation.magic.classify_format
                    when format_probe.format == "pdf"
            }

        }

        family structural {

            signal high_entropy {
                derive from operation.magic.entropy_probe
                    when entropy_probe.entropy > 7.0
            }

        }

        family risk {

            signal suspicious_pe {
                derive from operation.magic.inspect_magic
                    when magic_probe.matched
                      && magic_probe.magic[0] == 0x4D
                      && magic_probe.magic[1] == 0x5A
            }

        }

    }

    clinch {

        // --- PDF handling ---
        when signal.format.pdf_magic {
            tag += "type:pdf"
            emit Emit::PdfMagic
            run deferred PdfMagicHandler
            score risk += 1.0
        }

        when signal.format.classified_pdf {
            tag += "classified:pdf"
            score confidence += 2.0
        }

        // --- PE handling ---
        when signal.format.pe_magic {
            tag += "type:pe"
            emit Emit::PortableExecutable
            run deferred PeStaticAnalyzer
            score risk += 3.0
        }

        // --- Structural anomaly ---
        when signal.structural.high_entropy {
            tag += "anomaly:high_entropy"
            emit Emit::HighEntropy
            score risk += 5.0
        }

        // --- Risk escalation ---
        when signal.risk.suspicious_pe {
            tag += "risk:suspicious_pe"
            score threat = 9.0
        }

    }

}
}
