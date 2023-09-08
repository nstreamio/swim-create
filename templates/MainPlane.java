package {{self.name}};

import swim.api.plane.AbstractPlane;
import swim.kernel.Kernel;
import swim.server.ServerLoader;

public class MainPlane extends AbstractPlane {

    public static void main(String[] args) {
        final Kernel kernel = ServerLoader.loadServer();
        System.out.println("Starting server...");
        kernel.start();
        System.out.println("Running server...");
        kernel.run();
    }
}
