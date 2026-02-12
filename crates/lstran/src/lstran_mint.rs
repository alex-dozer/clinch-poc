/*

mint file for minting a pipeline component. can be any name but necessary for the main to have the file name.

*/

//needed import since these need to be in context for the pipeline
use common::data_objs::{Artifact, LuciusContext};

//macro just needs to be in context
use lucius_macro::lunch;

lunch! {

    component = lstran

    {

    /*

    Nothing happening with this right now. And I'm okay with it.

    basically just have meta and opening and closing braces and it should validate.

     */

    meta {
        name    = "structural_probe_poc"
        author  = "dozer-project"
        version = "0.2.0"
        scope   = any
    }

    operations {

        /*

        More validation needs to happen here. Like if i rename inspect_magic to inspooct_moogic
        I won't trip rust analyzer. This happened at one time because I tied existence checks to a
        macro called luop. I removed luop and i haven't added it. I will in the future.

        It is verbose by design, my thoughts being that if I was tired I only have to
        read the one line to get provenance for an op or signal. Could be annoying..

         */

        operation magic {
            do inspect_magic   output magic_probe
            do classify_format output format_probe
            do entropy_probe   output entropy_probe
        }

    }

    signals {

        family format {


            /*

            Signals use basic rust syntax for logic. I kept "when" since I
            felt it helped the idea that there is no "else".

             */

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

        /*

        runs and emits ideally would be enums in the end though
        I don't know if that is more complexity for not much gain.

        I'm brutally unaware of how comfortable this would be to
        people who don't code.


        I *think* it will be easy enough. But, I could just be way
        wrong....

         */

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
